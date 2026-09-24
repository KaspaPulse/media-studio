pub struct Spacing;

impl Spacing {
    pub const XS: f32 = 4.0;
    pub const SM: f32 = 8.0;
    pub const MD: f32 = 12.0;
    pub const LG: f32 = 16.0;
    pub const XL: f32 = 24.0;
    pub const XXL: f32 = 32.0;
}

pub struct Radius;

impl Radius {
    pub const SM: f32 = 8.0;
    pub const MD: f32 = 12.0;
    pub const LG: f32 = 16.0;
}

pub struct ControlSize;

impl ControlSize {
    pub const ICON: f32 = 36.0;
    pub const FIELD: f32 = 40.0;
    pub const PRIMARY: f32 = 44.0;
}

pub struct Typography;

impl Typography {
    pub const CAPTION: f32 = 12.0;
    pub const BODY: f32 = 16.0;
    pub const SECTION: f32 = 20.0;
    pub const TITLE: f32 = 30.0;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemanticColorRole {
    Accent,
    Success,
    Warning,
    Danger,
    Info,
    Surface,
    SurfaceElevated,
    TextPrimary,
    TextSecondary,
    BorderDefault,
    BorderFocused,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spacing_and_control_tokens_are_monotonic() {
        assert!(Spacing::XS < Spacing::SM);
        assert!(Spacing::SM < Spacing::MD);
        assert!(Spacing::MD < Spacing::LG);
        assert!(Spacing::LG < Spacing::XL);
        assert!(Spacing::XL < Spacing::XXL);

        assert!(ControlSize::ICON <= ControlSize::FIELD);
        assert!(ControlSize::FIELD <= ControlSize::PRIMARY);
    }

    #[test]
    fn typography_tokens_preserve_visual_hierarchy() {
        assert!(Typography::CAPTION < Typography::BODY);
        assert!(Typography::BODY < Typography::SECTION);
        assert!(Typography::SECTION < Typography::TITLE);
    }
}
