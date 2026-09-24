#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FocusKey(&'static str);

impl FocusKey {
    #[must_use]
    pub const fn new(value: &'static str) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FocusOrder<const N: usize> {
    keys: [FocusKey; N],
}

impl<const N: usize> FocusOrder<N> {
    #[must_use]
    pub const fn new(keys: [FocusKey; N]) -> Self {
        Self { keys }
    }

    #[must_use]
    pub const fn keys(&self) -> &[FocusKey; N] {
        &self.keys
    }

    #[must_use]
    pub fn next(&self, current: FocusKey) -> Option<FocusKey> {
        let index = self.keys.iter().position(|key| *key == current)?;
        self.keys.get(index + 1).copied()
    }

    #[must_use]
    pub fn previous(&self, current: FocusKey) -> Option<FocusKey> {
        let index = self.keys.iter().position(|key| *key == current)?;
        self.keys.get(index.checked_sub(1)?).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SOURCE: FocusKey = FocusKey::new("source");
    const PROFILE: FocusKey = FocusKey::new("profile");
    const OUTPUT: FocusKey = FocusKey::new("output");

    #[test]
    fn semantic_focus_order_is_explicit_and_not_visual_direction_dependent() {
        let order = FocusOrder::new([SOURCE, PROFILE, OUTPUT]);

        assert_eq!(order.next(SOURCE), Some(PROFILE));
        assert_eq!(order.next(PROFILE), Some(OUTPUT));
        assert_eq!(order.next(OUTPUT), None);

        assert_eq!(order.previous(OUTPUT), Some(PROFILE));
        assert_eq!(order.previous(PROFILE), Some(SOURCE));
        assert_eq!(order.previous(SOURCE), None);
    }

    #[test]
    fn focus_keys_are_stable_testable_identifiers() {
        assert_eq!(SOURCE.as_str(), "source");
        assert_eq!(order_keys(), ["source", "profile", "output"]);
    }

    fn order_keys() -> [&'static str; 3] {
        FocusOrder::new([SOURCE, PROFILE, OUTPUT])
            .keys()
            .map(FocusKey::as_str)
    }
}
