use anyhow::{Context, Result, bail, ensure};
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};
use std::env;
use std::fmt::Write as _;
use std::fs;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use std::process::Command;

const REPOSITORY: &str = "KaspaPulse/whatsapp-video-preparer";
const KSSS_RELEASE: &str = "v1.2.0";
const KSSS_SOURCE_SHA: &str = "967ed5068947a39961d5d5cc483ef65d25a61059";
const KSSS_POLICY_BUNDLE: &str = "3c1c8b449d736aba5fec5cffd688496b06d81f287f55fff4c3f42e36c3b58ec6";
const KSSS_RUNTIME_SHA256: &str =
    "38309d2ab8fa30096d99940f855e88173faa182e60db33f2a96b2d3408507430";
const KSSS_RUNTIME_SEQUENCE: u64 = 2;
const KSSS_TRUST_ROOT: &str = "ksss-trust-root-1";
const ACTIONS_ATTEST_SHA: &str = "1e69f48acb82d1966a394da916b4c1698aa569d6";
const CARGO_DENY_VERSION: &str = "0.20.2";
const CARGO_DENY_ACTION_SHA: &str = "3c6349835b2b7b196a839186cb8b78e02f7b5f25";

fn main() -> Result<()> {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        Some("verify") | None => verify_repository(),
        Some("digest") => {
            let path = args.next().context("digest requires a path")?;
            println!("{}", canonical_digest(&repo_root().join(path))?);
            Ok(())
        }
        Some("fetch-helpers") => {
            let platform = args.next().context("fetch-helpers requires a platform")?;
            let destination = args
                .next()
                .context("fetch-helpers requires a destination directory")?;
            fetch_helpers(&platform, &destination)
        }
        Some("verify-size-budget-package") => {
            let package = args
                .next()
                .context("verify-size-budget-package requires a package root")?;
            verify_size_budget_package(&package)
        }
        Some("package-windows") => {
            let binary = args.next().context("package-windows requires a binary")?;
            let output = args
                .next()
                .context("package-windows requires an output directory")?;
            package_windows(&binary, &output)
        }
        Some("package-macos") => {
            let platform = args.next().context("package-macos requires a platform")?;
            let binary = args.next().context("package-macos requires a binary")?;
            let output = args
                .next()
                .context("package-macos requires a dist directory")?;
            package_macos(&platform, &binary, &output)
        }
        Some("smoke") => {
            let binary = args.next().context("smoke requires a binary path")?;
            smoke_binary(&binary)
        }
        Some(other) => bail!("unknown xtask command: {other}"),
    }
}
fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask must live under repository root")
        .to_path_buf()
}

fn verify_repository() -> Result<()> {
    let root = repo_root();
    verify_required_files(&root)?;
    verify_rust_only(&root)?;
    verify_size_budget(&root)?;
    verify_ksss(&root)?;
    verify_helper_manifest(&root)?;
    verify_dependency_policy(&root)?;
    println!("WVP_REPOSITORY_GATE=PASS");
    println!("RUST_OWNED_IMPLEMENTATION=100_PERCENT");
    println!("KSSS_RELEASE={KSSS_RELEASE}");
    println!("WHATSAPP_SIZE_POLICY=PASS");
    println!("WORKFLOW_DECLARATIVE_ORCHESTRATION_ONLY=PASS");
    println!("RUST_ONLY_GATE=STRICT_PASS");
    println!("CARGO_DENY_POLICY=PASS version={CARGO_DENY_VERSION}");
    Ok(())
}

fn verify_required_files(root: &Path) -> Result<()> {
    for relative in [
        "Cargo.toml",
        "Cargo.lock",
        "rust-toolchain.toml",
        "src/main.rs",
        "src/media.rs",
        "xtask/Cargo.toml",
        "deny.toml",
        ".github/workflows/rust-policy.yml",
        ".security/ksss/ksss-adoption.json",
        ".security/ksss/trust-policy.json",
        ".security/ksss/helper-supply-chain.json",
        "docs/continuity/DURABLE_LOCAL_CHECKPOINTING.md",
    ] {
        ensure!(
            root.join(relative).is_file(),
            "required file missing: {relative}"
        );
    }
    Ok(())
}
fn verify_rust_only(root: &Path) -> Result<()> {
    let forbidden_names = [
        "pyproject.toml",
        "requirements.txt",
        "build.ps1",
        "build_macos.sh",
    ];
    for name in forbidden_names {
        ensure!(
            !root.join(name).exists(),
            "legacy code surface returned: {name}"
        );
    }

    let mut rust_files = 0_usize;
    walk_owned_files(root, root, &mut |path| {
        let extension = path
            .extension()
            .and_then(|value| value.to_str())
            .unwrap_or("");
        if extension.eq_ignore_ascii_case("rs") {
            rust_files += 1;
        }
        ensure!(
            !matches!(
                extension.to_ascii_lowercase().as_str(),
                "py" | "pyw"
                    | "pyc"
                    | "ps1"
                    | "sh"
                    | "bash"
                    | "zsh"
                    | "js"
                    | "jsx"
                    | "ts"
                    | "tsx"
                    | "cmd"
                    | "bat"
            ),
            "non-Rust owned code file is forbidden: {}",
            path.strip_prefix(root).unwrap_or(path).display()
        );
        Ok(())
    })?;
    ensure!(
        rust_files >= 9,
        "expected Rust application plus xtask sources"
    );
    verify_workflows_are_declarative(root)?;
    Ok(())
}
fn walk_owned_files<F>(root: &Path, current: &Path, visitor: &mut F) -> Result<()>
where
    F: FnMut(&Path) -> Result<()>,
{
    for entry in
        fs::read_dir(current).with_context(|| format!("failed to read {}", current.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let relative = path.strip_prefix(root).unwrap_or(&path);
            if matches!(
                relative.to_string_lossy().replace('\\', "/").as_str(),
                ".git" | "target" | ".artifacts" | ".ksss-cache"
            ) {
                continue;
            }
            walk_owned_files(root, &path, visitor)?;
        } else {
            visitor(&path)?;
        }
    }
    Ok(())
}

fn verify_workflows_are_declarative(root: &Path) -> Result<()> {
    let workflows = root.join(".github/workflows");
    let mut saw_attest = false;
    let mut saw_cargo_deny = false;
    for entry in fs::read_dir(workflows)? {
        let path = entry?.path();
        if !path.is_file() {
            continue;
        }
        let text = fs::read_to_string(&path)?;
        let lower = text.to_ascii_lowercase();
        for forbidden in [
            "setup-python",
            "python ",
            ".ps1",
            "actions/attest-build-provenance@",
            "actions/attest-sbom@",
        ] {
            ensure!(
                !lower.contains(forbidden),
                "workflow {} still references forbidden surface {forbidden}",
                path.display()
            );
        }
        ensure!(
            !lower.contains("shell:"),
            "workflow {} must rely on runner defaults; explicit shell selectors are forbidden",
            path.display()
        );
        let lines: Vec<_> = text.lines().collect();
        verify_workflow_run_blocks(&path, &lines)?;
        for line in &lines {
            let trimmed = line.trim_start_matches([' ', '-']).trim();
            let Some(spec) = trimmed.strip_prefix("uses:") else {
                continue;
            };
            let spec = spec.trim();
            ensure!(
                !spec.starts_with("./"),
                "repository-local workflow actions are forbidden: {spec}"
            );
            let (_, reference) = spec
                .split_once('@')
                .with_context(|| format!("workflow action is missing a ref: {spec}"))?;
            let reference = reference.split_whitespace().next().unwrap_or("");
            ensure!(
                reference.len() == 40 && reference.bytes().all(|byte| byte.is_ascii_hexdigit()),
                "workflow action must use a full commit SHA: {spec}"
            );
            if spec.starts_with("actions/attest@") {
                ensure!(
                    reference == ACTIONS_ATTEST_SHA,
                    "actions/attest must be pinned to the approved v4.2.2 commit"
                );
                saw_attest = true;
            }
            if spec.starts_with("EmbarkStudios/cargo-deny-action@") {
                ensure!(
                    reference == CARGO_DENY_ACTION_SHA,
                    "cargo-deny-action must be pinned to the approved v2.1.1 commit"
                );
                saw_cargo_deny = true;
            }
        }
    }
    ensure!(saw_attest, "actions/attest v4.2.2 pin is required");
    ensure!(saw_cargo_deny, "cargo-deny-action v2.1.1 pin is required");
    Ok(())
}

fn verify_workflow_run_blocks(path: &Path, lines: &[&str]) -> Result<()> {
    let mut index = 0_usize;
    while index < lines.len() {
        let line = lines[index];
        let trimmed = line.trim_start();
        let indent = line.len() - trimmed.len();
        let Some(run) = trimmed.strip_prefix("run:") else {
            index += 1;
            continue;
        };
        let run = run.trim();
        if matches!(run, "|" | "|-" | ">" | ">-") {
            index += 1;
            while index < lines.len() {
                let nested = lines[index];
                if nested.trim().is_empty() {
                    index += 1;
                    continue;
                }
                let nested_trimmed = nested.trim_start();
                let nested_indent = nested.len() - nested_trimmed.len();
                if nested_indent <= indent {
                    break;
                }
                verify_workflow_command(path, nested_trimmed.trim())?;
                index += 1;
            }
            continue;
        }
        verify_workflow_command(path, run)?;
        index += 1;
    }
    Ok(())
}

fn verify_workflow_command(path: &Path, command: &str) -> Result<()> {
    let sanitized = strip_github_expressions(command)?;
    for forbidden in ["&&", "||", ";", "`", "$", "|", ">", "<", "="] {
        ensure!(
            !sanitized.contains(forbidden),
            "workflow {} contains inline executable logic: {command}",
            path.display()
        );
    }
    ensure!(
        ["cargo ", "rustup ", "rustc "]
            .iter()
            .any(|prefix| sanitized.starts_with(prefix)),
        "workflow {} run command is not approved orchestration: {command}",
        path.display()
    );
    Ok(())
}

fn strip_github_expressions(command: &str) -> Result<String> {
    let mut output = String::new();
    let mut rest = command;
    while let Some(start) = rest.find("${{") {
        output.push_str(&rest[..start]);
        let expression = &rest[start + 3..];
        let end = expression
            .find("}}")
            .context("unterminated GitHub expression in workflow run command")?;
        output.push_str("GITHUB_EXPRESSION");
        rest = &expression[end + 2..];
    }
    output.push_str(rest);
    Ok(output)
}

fn verify_dependency_policy(root: &Path) -> Result<()> {
    let policy = fs::read_to_string(root.join("deny.toml"))?;
    for required in [
        "x86_64-pc-windows-msvc",
        "aarch64-apple-darwin",
        "x86_64-apple-darwin",
        "unknown-registry = \"deny\"",
        "unknown-git = \"deny\"",
        "yanked = \"deny\"",
        "unmaintained = \"workspace\"",
        "unsound = \"all\"",
        "\"BSL-1.0\"",
        "\"CC0-1.0\"",
    ] {
        ensure!(
            policy.contains(required),
            "deny.toml missing required policy: {required}"
        );
    }
    let workflow = fs::read_to_string(root.join(".github/workflows/rust-policy.yml"))?;
    ensure!(
        workflow.contains("command-arguments: advisories licenses bans sources"),
        "Rust policy workflow must execute all four cargo-deny checks"
    );
    Ok(())
}

fn verify_size_budget(root: &Path) -> Result<()> {
    let profile_source = fs::read_to_string(root.join("src/export_profile.rs"))?;
    let media_source = fs::read_to_string(root.join("src/media.rs"))?;
    let hard = parse_u64_const(&profile_source, "WHATSAPP_HARD_LIMIT_BYTES")?;
    let target = parse_u64_const(&profile_source, "WHATSAPP_TARGET_BYTES")?;
    ensure!(
        hard == 10_000_000,
        "WhatsApp hard limit must be 10,000,000 bytes"
    );
    ensure!(
        target == 9_500_000,
        "WhatsApp target must retain the 5% safety margin"
    );
    ensure!(target < hard, "target must be below the hard limit");
    ensure!(
        media_source.contains("fs::metadata(output)?.len()"),
        "encoded clip size must be measured from the actual output"
    );
    Ok(())
}

fn parse_u64_const(source: &str, name: &str) -> Result<u64> {
    let marker = format!("pub const {name}: u64 = ");
    let line = source
        .lines()
        .find(|line| line.trim_start().starts_with(&marker))
        .with_context(|| format!("missing constant {name}"))?;
    let raw = line
        .trim()
        .strip_prefix(&marker)
        .context("invalid constant declaration")?
        .trim_end_matches(';')
        .replace('_', "");
    Ok(raw.parse()?)
}
fn verify_ksss(root: &Path) -> Result<()> {
    let adoption = read_json(&root.join(".security/ksss/ksss-adoption.json"))?;
    let trust = read_json(&root.join(".security/ksss/trust-policy.json"))?;
    let policy = read_json(&root.join(".security/ksss/repository-policy.json"))?;
    let strengthening = read_json(&root.join(".security/ksss/local-strengthening.json"))?;

    expect_string(&adoption, "repository", REPOSITORY)?;
    expect_string(&adoption, "ksss_release", KSSS_RELEASE)?;
    expect_string(&adoption, "ksss_source_sha", KSSS_SOURCE_SHA)?;
    expect_string(&adoption, "policy_bundle_digest", KSSS_POLICY_BUNDLE)?;
    expect_string(&adoption, "repository_profile", "medium")?;

    expect_string(&trust, "ksss_release", KSSS_RELEASE)?;
    expect_string(&trust, "ksss_source_sha", KSSS_SOURCE_SHA)?;
    expect_string(&trust, "policy_bundle_digest", KSSS_POLICY_BUNDLE)?;
    expect_string(&trust, "runtime_artifact_sha256", KSSS_RUNTIME_SHA256)?;
    expect_string(&trust, "trust_root_version", KSSS_TRUST_ROOT)?;
    expect_u64(&trust, "runtime_sequence", KSSS_RUNTIME_SEQUENCE)?;

    ensure!(
        policy.pointer("/controls/risk_floor_enforcement") == Some(&Value::Bool(true)),
        "KSSS risk floor must remain enabled"
    );
    ensure!(
        strengthening.pointer("/controls/WVP-RUST-002/workflow_orchestration_only")
            == Some(&Value::Bool(true)),
        "strict workflow orchestration control must remain enabled"
    );
    ensure!(
        strengthening.pointer("/controls/WVP-RUST-002/external_actions_full_sha")
            == Some(&Value::Bool(true)),
        "external Actions must remain full-SHA pinned"
    );
    ensure!(
        strengthening
            .pointer("/controls/WVP-SUPPLY-001/cargo_deny_version")
            .and_then(Value::as_str)
            == Some(CARGO_DENY_VERSION),
        "cargo-deny policy version mismatch"
    );
    ensure!(
        strengthening
            .pointer("/controls/WVP-SUPPLY-001/yanked")
            .and_then(Value::as_str)
            == Some("deny"),
        "cargo-deny yanked policy must remain deny"
    );
    ensure!(
        strengthening
            .pointer("/controls/WVP-SUPPLY-001/unsound")
            .and_then(Value::as_str)
            == Some("all"),
        "cargo-deny unsound policy must remain all"
    );
    ensure!(
        strengthening
            .pointer("/controls/WVP-SUPPLY-001/unmaintained")
            .and_then(Value::as_str)
            == Some("workspace"),
        "cargo-deny unmaintained policy must remain workspace"
    );
    ensure!(
        strengthening
            .pointer("/controls/WVP-SUPPLY-001/advisory_ignores")
            .and_then(Value::as_array)
            .is_some_and(Vec::is_empty),
        "cargo-deny advisory ignore list must remain empty"
    );
    ensure!(
        strengthening
            .pointer("/controls/WVP-ATTEST-001/action_sha")
            .and_then(Value::as_str)
            == Some(ACTIONS_ATTEST_SHA),
        "actions/attest policy SHA mismatch"
    );
    verify_bound_digest(
        root,
        &adoption,
        "risk_classification_digest",
        "risk-classification.json",
    )?;
    verify_bound_digest(
        root,
        &adoption,
        "applicability_digest",
        "applicability.json",
    )?;
    Ok(())
}
fn verify_bound_digest(root: &Path, adoption: &Value, field: &str, file: &str) -> Result<()> {
    let expected = adoption
        .get(field)
        .and_then(Value::as_str)
        .with_context(|| format!("missing adoption field {field}"))?;
    let actual = canonical_digest(&root.join(".security/ksss").join(file))?;
    ensure!(expected == actual, "{field} does not match {file}");
    Ok(())
}

fn read_json(path: &Path) -> Result<Value> {
    let bytes = fs::read(path).with_context(|| format!("failed to read {}", path.display()))?;
    serde_json::from_slice(&bytes).with_context(|| format!("invalid JSON: {}", path.display()))
}

fn expect_string(value: &Value, field: &str, expected: &str) -> Result<()> {
    let actual = value.get(field).and_then(Value::as_str);
    ensure!(actual == Some(expected), "{field} mismatch");
    Ok(())
}

fn expect_u64(value: &Value, field: &str, expected: u64) -> Result<()> {
    let actual = value.get(field).and_then(Value::as_u64);
    ensure!(actual == Some(expected), "{field} mismatch");
    Ok(())
}
fn canonical_digest(path: &Path) -> Result<String> {
    let value = read_json(path)?;
    let normalized = normalize(value);
    let bytes = serde_json::to_vec(&normalized)?;
    let digest = Sha256::digest(bytes);
    Ok(bytes_to_hex(&digest))
}

fn bytes_to_hex(bytes: &[u8]) -> String {
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        write!(&mut output, "{byte:02x}").expect("writing to a String cannot fail");
    }
    output
}

fn normalize(value: Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut entries: Vec<_> = map.into_iter().collect();
            entries.sort_by(|left, right| left.0.cmp(&right.0));
            let mut sorted = Map::new();
            for (key, value) in entries {
                sorted.insert(key, normalize(value));
            }
            Value::Object(sorted)
        }
        Value::Array(values) => Value::Array(values.into_iter().map(normalize).collect()),
        other => other,
    }
}

fn verify_helper_manifest(root: &Path) -> Result<()> {
    let manifest = read_json(&root.join(".security/ksss/helper-supply-chain.json"))?;
    expect_string(&manifest, "schema", "wvp-helper-supply-chain-v1")?;
    expect_string(&manifest, "ffmpeg_release", "n8.1.2-1")?;
    expect_string(&manifest, "yt_dlp_release", "2026.08.19")?;
    let platforms = manifest
        .get("platforms")
        .and_then(Value::as_object)
        .context("helper manifest platforms must be an object")?;
    for platform in ["windows-x64", "macos-arm64", "macos-x64"] {
        let assets = platforms
            .get(platform)
            .and_then(Value::as_array)
            .with_context(|| format!("helper platform missing: {platform}"))?;
        ensure!(
            assets.len() == 3,
            "{platform} must define exactly three helpers"
        );
        for asset in assets {
            verify_helper_asset(asset)?;
        }
    }
    Ok(())
}
fn verify_helper_asset(asset: &Value) -> Result<()> {
    let output = asset
        .get("output_name")
        .and_then(Value::as_str)
        .context("helper output_name missing")?;
    let url = asset
        .get("url")
        .and_then(Value::as_str)
        .context("helper url missing")?;
    let digest = asset
        .get("sha256")
        .and_then(Value::as_str)
        .context("helper sha256 missing")?;
    ensure!(
        Path::new(output).components().count() == 1,
        "helper output_name must be a plain file name: {output}"
    );
    ensure!(
        url.starts_with("https://github.com/"),
        "helper URL must use GitHub HTTPS"
    );
    ensure!(
        url.contains("/n8.1.2-1/") || url.contains("/2026.08.19/"),
        "helper URL is not pinned to an approved release: {url}"
    );
    ensure!(
        digest.len() == 64 && digest.bytes().all(|byte| byte.is_ascii_hexdigit()),
        "helper SHA-256 must be 64 hexadecimal characters"
    );
    Ok(())
}
fn fetch_helpers(platform: &str, destination: &str) -> Result<()> {
    let root = repo_root();
    verify_helper_manifest(&root)?;
    ensure!(
        current_platform() == Some(platform),
        "helper platform {platform} does not match this runner"
    );
    let manifest = read_json(&root.join(".security/ksss/helper-supply-chain.json"))?;
    let assets = manifest
        .pointer(&format!("/platforms/{platform}"))
        .and_then(Value::as_array)
        .with_context(|| format!("unsupported helper platform: {platform}"))?;
    let destination = PathBuf::from(destination);
    fs::create_dir_all(&destination)
        .with_context(|| format!("failed to create {}", destination.display()))?;
    let client = reqwest::blocking::Client::builder()
        .user_agent("KaspaPulse-whatsapp-video-preparer-xtask/2.0.0")
        .build()
        .context("failed to construct helper download client")?;
    for asset in assets {
        fetch_helper_asset(&client, asset, &destination)?;
    }
    Ok(())
}
fn fetch_helper_asset(
    client: &reqwest::blocking::Client,
    asset: &Value,
    destination: &Path,
) -> Result<()> {
    verify_helper_asset(asset)?;
    let output = asset["output_name"]
        .as_str()
        .context("missing output_name")?;
    let url = asset["url"].as_str().context("missing url")?;
    let expected = asset["sha256"].as_str().context("missing sha256")?;
    let target = destination.join(output);
    if target.exists() {
        let actual = sha256_file(&target)?;
        ensure!(
            actual == expected,
            "existing helper has unexpected digest: {}",
            target.display()
        );
        set_executable(&target)?;
        verify_helper_process(&target)?;
        println!("HELPER_REUSED={} sha256={actual}", target.display());
        return Ok(());
    }
    let temporary = destination.join(format!(".{output}.partial-{}", std::process::id()));
    ensure!(!temporary.exists(), "temporary helper path already exists");
    let mut response = client
        .get(url)
        .send()
        .with_context(|| format!("failed to download {url}"))?
        .error_for_status()
        .with_context(|| format!("helper download returned an error: {url}"))?;
    let mut file = fs::File::create(&temporary)
        .with_context(|| format!("failed to create {}", temporary.display()))?;
    std::io::copy(&mut response, &mut file)
        .with_context(|| format!("failed to write {}", temporary.display()))?;
    file.sync_all()?;
    drop(file);
    let actual = sha256_file(&temporary)?;
    if actual != expected {
        let _ = fs::remove_file(&temporary);
        bail!("helper digest mismatch for {output}: expected {expected}, actual {actual}");
    }
    fs::rename(&temporary, &target)
        .with_context(|| format!("failed to publish verified helper {}", target.display()))?;
    set_executable(&target)?;
    verify_helper_process(&target)?;
    println!("HELPER_FETCHED={} sha256={actual}", target.display());
    Ok(())
}
fn sha256_file(path: &Path) -> Result<String> {
    let file =
        fs::File::open(path).with_context(|| format!("failed to open {}", path.display()))?;
    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut buffer = vec![0_u8; 64 * 1024];
    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }
    let digest = hasher.finalize();
    Ok(bytes_to_hex(&digest))
}

fn verify_helper_process(path: &Path) -> Result<()> {
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("");
    let argument = if file_name.starts_with("yt-dlp") {
        "--version"
    } else {
        "-version"
    };
    let status = Command::new(path)
        .arg(argument)
        .status()
        .with_context(|| format!("failed to execute helper {}", path.display()))?;
    ensure!(
        status.success(),
        "helper self-check failed: {}",
        path.display()
    );
    Ok(())
}
fn current_platform() -> Option<&'static str> {
    match (env::consts::OS, env::consts::ARCH) {
        ("windows", "x86_64") => Some("windows-x64"),
        ("macos", "aarch64") => Some("macos-arm64"),
        ("macos", "x86_64") => Some("macos-x64"),
        _ => None,
    }
}

fn set_executable(path: &Path) -> Result<()> {
    let metadata = fs::metadata(path)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut permissions = metadata.permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(path, permissions)?;
    }
    #[cfg(not(unix))]
    let _ = metadata;
    Ok(())
}

fn resolve_repo_path(value: &str) -> PathBuf {
    let path = PathBuf::from(value);
    if path.is_absolute() {
        path
    } else {
        repo_root().join(path)
    }
}

fn verify_size_budget_package(package: &str) -> Result<()> {
    let platform = current_platform()
        .context("size-budget package verification requires a supported runner")?;
    let package = resolve_repo_path(package);
    ensure!(
        package.is_dir(),
        "package root missing: {}",
        package.display()
    );
    let resources = if platform == "windows-x64" {
        package.join("resources")
    } else {
        package.join("Contents/Resources")
    };
    verify_packaged_helpers(platform, &resources)?;
    let ffmpeg = resources.join(if platform == "windows-x64" {
        "ffmpeg.exe"
    } else {
        "ffmpeg"
    });
    let ffprobe = resources.join(if platform == "windows-x64" {
        "ffprobe.exe"
    } else {
        "ffprobe"
    });
    let cargo = env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    let mut command = Command::new(cargo);
    command
        .current_dir(repo_root())
        .args([
            "test",
            "--locked",
            "--test",
            "size_budget",
            "--",
            "--ignored",
            "--nocapture",
        ])
        .env("WVP_FFMPEG", &ffmpeg)
        .env("WVP_FFPROBE", &ffprobe);
    run_checked(&mut command)?;
    println!(
        "SIZE_BUDGET_PACKAGE_TEST=PASS platform={platform} package={}",
        package.display()
    );
    Ok(())
}

fn verify_packaged_helpers(platform: &str, resources: &Path) -> Result<()> {
    ensure!(
        resources.is_dir(),
        "package resources missing: {}",
        resources.display()
    );
    let manifest = read_json(&repo_root().join(".security/ksss/helper-supply-chain.json"))?;
    let assets = manifest
        .pointer(&format!("/platforms/{platform}"))
        .and_then(Value::as_array)
        .with_context(|| format!("unsupported helper platform: {platform}"))?;
    for asset in assets {
        let output = asset
            .get("output_name")
            .and_then(Value::as_str)
            .context("helper output_name missing")?;
        let expected = asset
            .get("sha256")
            .and_then(Value::as_str)
            .context("helper sha256 missing")?;
        let target = resources.join(output);
        ensure!(
            target.is_file(),
            "packaged helper missing: {}",
            target.display()
        );
        let actual = sha256_file(&target)?;
        ensure!(
            actual == expected,
            "packaged helper digest mismatch for {output}: expected {expected}, actual {actual}"
        );
    }
    Ok(())
}

fn package_windows(binary: &str, output: &str) -> Result<()> {
    ensure!(
        current_platform() == Some("windows-x64"),
        "package-windows must run on windows-x64"
    );
    let binary = resolve_repo_path(binary);
    ensure!(
        binary.is_file(),
        "release binary missing: {}",
        binary.display()
    );
    let output = resolve_repo_path(output);
    ensure!(
        !output.exists(),
        "package output already exists: {}",
        output.display()
    );
    fs::create_dir_all(output.join("resources"))?;
    fs::copy(&binary, output.join("WhatsAppVideoPreparer.exe"))?;
    let resources = output.join("resources");
    fetch_helpers("windows-x64", &resources.to_string_lossy())?;
    write_sha256sums(&output)?;
    println!("WINDOWS_PACKAGE={}", output.display());
    Ok(())
}
fn package_macos(platform: &str, binary: &str, dist: &str) -> Result<()> {
    ensure!(
        matches!(platform, "macos-arm64" | "macos-x64"),
        "unsupported macOS platform: {platform}"
    );
    ensure!(
        current_platform() == Some(platform),
        "macOS package platform mismatch"
    );
    let binary = resolve_repo_path(binary);
    ensure!(
        binary.is_file(),
        "release binary missing: {}",
        binary.display()
    );
    let dist = resolve_repo_path(dist);
    fs::create_dir_all(&dist)?;
    let package_root = dist.join(format!("WhatsAppVideoPreparer-{platform}"));
    ensure!(
        !package_root.exists(),
        "package output already exists: {}",
        package_root.display()
    );
    let app = package_root.join("WhatsAppVideoPreparer.app");
    let macos = app.join("Contents/MacOS");
    let resources = app.join("Contents/Resources");
    fs::create_dir_all(&macos)?;
    fs::create_dir_all(&resources)?;
    let app_binary = macos.join("WhatsAppVideoPreparer");
    fs::copy(&binary, &app_binary)?;
    set_executable(&app_binary)?;
    fs::copy(
        repo_root().join("assets/app_icon.icns"),
        resources.join("app_icon.icns"),
    )?;
    fetch_helpers(platform, &resources.to_string_lossy())?;
    let plist = r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "https://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleDisplayName</key><string>WhatsApp Video Preparer</string>
  <key>CFBundleExecutable</key><string>WhatsAppVideoPreparer</string>
  <key>CFBundleIdentifier</key><string>com.kaspapulse.whatsapp-video-preparer</string>
  <key>CFBundleIconFile</key><string>app_icon.icns</string>
  <key>CFBundleName</key><string>WhatsApp Video Preparer</string>
  <key>CFBundlePackageType</key><string>APPL</string>
  <key>CFBundleShortVersionString</key><string>2.0.0</string>
  <key>CFBundleVersion</key><string>2.0.0</string>
  <key>LSMinimumSystemVersion</key><string>12.0</string>
  <key>NSHighResolutionCapable</key><true/>
</dict>
</plist>
"#;
    fs::write(app.join("Contents/Info.plist"), plist)?;
    run_checked(
        Command::new("codesign")
            .args(["--force", "--deep", "--sign", "-"])
            .arg(&app),
    )?;
    run_checked(
        Command::new("codesign")
            .args(["--verify", "--deep", "--strict"])
            .arg(&app),
    )?;
    write_sha256sums(&package_root)?;
    let dmg = dist.join(format!("WhatsAppVideoPreparer-{platform}.dmg"));
    ensure!(!dmg.exists(), "DMG already exists: {}", dmg.display());
    run_checked(
        Command::new("hdiutil")
            .args([
                "create",
                "-volname",
                "WhatsApp Video Preparer",
                "-srcfolder",
            ])
            .arg(&app)
            .args(["-format", "UDZO"])
            .arg(&dmg),
    )?;
    let digest = sha256_file(&dmg)?;
    fs::write(
        dist.join(format!("WhatsAppVideoPreparer-{platform}.dmg.sha256")),
        format!(
            "{digest}  {}\n",
            dmg.file_name().unwrap_or_default().to_string_lossy()
        ),
    )?;
    println!("MACOS_PACKAGE={}", package_root.display());
    println!("MACOS_DMG={} SHA256={digest}", dmg.display());
    Ok(())
}

fn run_checked(command: &mut Command) -> Result<()> {
    let debug = format!("{command:?}");
    let status = command
        .status()
        .with_context(|| format!("failed to start {debug}"))?;
    ensure!(status.success(), "command failed: {debug}");
    Ok(())
}
fn smoke_binary(binary: &str) -> Result<()> {
    let binary = resolve_repo_path(binary);
    ensure!(
        binary.is_file(),
        "smoke binary missing: {}",
        binary.display()
    );
    let mut child = Command::new(&binary)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .with_context(|| format!("failed to start {}", binary.display()))?;
    let pid = child.id();
    std::thread::sleep(std::time::Duration::from_secs(6));
    if let Some(status) = child.try_wait()? {
        bail!("GUI exited early with status {status}");
    }
    child.kill()?;
    let _ = child.wait();
    println!("GUI_SMOKE=PASS PID={pid} ALIVE_AFTER_SECONDS=6");
    Ok(())
}

fn write_sha256sums(root: &Path) -> Result<()> {
    let mut files = Vec::new();
    collect_package_files(root, root, &mut files)?;
    files.sort();
    let mut output = String::new();
    for relative in files {
        if relative == Path::new("SHA256SUMS") {
            continue;
        }
        let digest = sha256_file(&root.join(&relative))?;
        writeln!(
            &mut output,
            "{digest}  {}",
            relative.to_string_lossy().replace('\\', "/")
        )?;
    }
    fs::write(root.join("SHA256SUMS"), output)?;
    Ok(())
}
fn collect_package_files(root: &Path, current: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
    for entry in fs::read_dir(current)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_package_files(root, &path, files)?;
        } else if path.is_file() {
            files.push(path.strip_prefix(root)?.to_path_buf());
        }
    }
    Ok(())
}
