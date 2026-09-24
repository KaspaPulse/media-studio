fn main() {
    println!("cargo:rerun-if-changed=assets/app_icon.ico");

    #[cfg(windows)]
    {
        let mut resource = winresource::WindowsResource::new();
        resource.set_icon("assets/app_icon.ico");
        resource.set("ProductName", "KaspaPulse Media Studio");
        resource.set("FileDescription", "KaspaPulse Media Studio");
        resource.set("CompanyName", "KaspaPulse");
        resource.set("LegalCopyright", "KaspaPulse");
        resource
            .compile()
            .expect("failed to compile Windows executable resources");
    }
}
