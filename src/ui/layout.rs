#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutClass {
    Compact,
    Standard,
    Wide,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LayoutBreakpoints {
    compact_max: f32,
    standard_max: f32,
}

impl LayoutBreakpoints {
    #[must_use]
    pub fn new(compact_max: f32, standard_max: f32) -> Option<Self> {
        if !compact_max.is_finite()
            || !standard_max.is_finite()
            || compact_max <= 0.0
            || compact_max >= standard_max
        {
            return None;
        }
        Some(Self {
            compact_max,
            standard_max,
        })
    }

    #[must_use]
    pub fn classify(self, width: f32) -> Option<LayoutClass> {
        if !width.is_finite() || width < 0.0 {
            return None;
        }
        if width <= self.compact_max {
            Some(LayoutClass::Compact)
        } else if width <= self.standard_max {
            Some(LayoutClass::Standard)
        } else {
            Some(LayoutClass::Wide)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn caller_tuned_breakpoints_produce_semantic_layout_classes() {
        let breakpoints = LayoutBreakpoints::new(760.0, 1_200.0).unwrap();
        assert_eq!(breakpoints.classify(600.0), Some(LayoutClass::Compact));
        assert_eq!(breakpoints.classify(1_000.0), Some(LayoutClass::Standard));
        assert_eq!(breakpoints.classify(1_400.0), Some(LayoutClass::Wide));
    }

    #[test]
    fn invalid_breakpoints_or_widths_fail_closed() {
        assert!(LayoutBreakpoints::new(0.0, 1_200.0).is_none());
        assert!(LayoutBreakpoints::new(1_200.0, 760.0).is_none());

        let breakpoints = LayoutBreakpoints::new(760.0, 1_200.0).unwrap();
        assert!(breakpoints.classify(f32::NAN).is_none());
        assert!(breakpoints.classify(-1.0).is_none());
    }
}
