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

    fn strictly_increasing(values: &[f32]) -> bool {
        values.windows(2).all(|pair| pair[0] < pair[1])
    }

    fn non_decreasing(values: &[f32]) -> bool {
        values.windows(2).all(|pair| pair[0] <= pair[1])
    }

    #[test]
    fn spacing_and_control_tokens_are_monotonic() {
        let spacing = [
            Spacing::XS,
            Spacing::SM,
            Spacing::MD,
            Spacing::LG,
            Spacing::XL,
            Spacing::XXL,
        ];
        let controls = [ControlSize::ICON, ControlSize::FIELD, ControlSize::PRIMARY];

        assert!(strictly_increasing(&spacing));
        assert!(non_decreasing(&controls));
    }

    #[test]
    fn typography_tokens_preserve_visual_hierarchy() {
        let typography = [
            Typography::CAPTION,
            Typography::BODY,
            Typography::SECTION,
            Typography::TITLE,
        ];

        assert!(strictly_increasing(&typography));
    }
}
