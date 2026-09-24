use iced::event;
use iced::keyboard::{Key, Modifiers, key::Named};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyboardCommand {
    FocusNext,
    FocusPrevious,
    ActivateFocused,
    DismissTransient,
}

#[must_use]
pub fn keyboard_command(
    key: &Key,
    modifiers: Modifiers,
    repeat: bool,
    status: event::Status,
) -> Option<KeyboardCommand> {
    if repeat {
        return None;
    }

    if matches!(key, Key::Named(Named::Escape)) {
        return Some(KeyboardCommand::DismissTransient);
    }

    if status == event::Status::Captured {
        return None;
    }

    match key {
        Key::Named(Named::Tab) if modifiers.shift() => Some(KeyboardCommand::FocusPrevious),
        Key::Named(Named::Tab) => Some(KeyboardCommand::FocusNext),
        Key::Named(Named::Enter | Named::Space) => Some(KeyboardCommand::ActivateFocused),
        _ => None,
    }
}

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

#[must_use]
pub fn cycle_available<T, const N: usize>(
    order: &[T; N],
    current: Option<T>,
    reverse: bool,
    mut available: impl FnMut(T) -> bool,
) -> Option<T>
where
    T: Copy + PartialEq,
{
    if N == 0 {
        return None;
    }

    let start = current
        .and_then(|current| order.iter().position(|candidate| *candidate == current))
        .unwrap_or_else(|| if reverse { 0 } else { N - 1 });

    for step in 1..=N {
        let index = if reverse {
            (start + N - (step % N)) % N
        } else {
            (start + step) % N
        };
        let candidate = order[index];
        if available(candidate) {
            return Some(candidate);
        }
    }

    None
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

    #[test]
    fn keyboard_commands_are_emitted_only_for_uncaptured_non_repeated_keys() {
        assert_eq!(
            keyboard_command(
                &Key::Named(Named::Tab),
                Modifiers::NONE,
                false,
                event::Status::Ignored,
            ),
            Some(KeyboardCommand::FocusNext)
        );
        assert_eq!(
            keyboard_command(
                &Key::Named(Named::Tab),
                Modifiers::SHIFT,
                false,
                event::Status::Ignored,
            ),
            Some(KeyboardCommand::FocusPrevious)
        );
        assert_eq!(
            keyboard_command(
                &Key::Named(Named::Enter),
                Modifiers::NONE,
                false,
                event::Status::Ignored,
            ),
            Some(KeyboardCommand::ActivateFocused)
        );
        assert_eq!(
            keyboard_command(
                &Key::Named(Named::Space),
                Modifiers::NONE,
                false,
                event::Status::Ignored,
            ),
            Some(KeyboardCommand::ActivateFocused)
        );
        assert_eq!(
            keyboard_command(
                &Key::Named(Named::Escape),
                Modifiers::NONE,
                false,
                event::Status::Ignored,
            ),
            Some(KeyboardCommand::DismissTransient)
        );
        assert_eq!(
            keyboard_command(
                &Key::Named(Named::Tab),
                Modifiers::NONE,
                true,
                event::Status::Ignored,
            ),
            None
        );
        assert_eq!(
            keyboard_command(
                &Key::Named(Named::Tab),
                Modifiers::NONE,
                false,
                event::Status::Captured,
            ),
            None
        );
        assert_eq!(
            keyboard_command(
                &Key::Named(Named::Escape),
                Modifiers::NONE,
                false,
                event::Status::Captured,
            ),
            Some(KeyboardCommand::DismissTransient)
        );
    }

    #[test]
    fn cycle_available_wraps_and_skips_unavailable_targets() {
        let order = [1_u8, 2, 3, 4];

        assert_eq!(
            cycle_available(&order, None, false, |candidate| candidate != 2),
            Some(1)
        );
        assert_eq!(
            cycle_available(&order, Some(1), false, |candidate| candidate != 2),
            Some(3)
        );
        assert_eq!(
            cycle_available(&order, Some(3), true, |candidate| candidate != 2),
            Some(1)
        );
        assert_eq!(cycle_available(&order, Some(4), false, |_| false), None);
    }

    fn order_keys() -> [&'static str; 3] {
        FocusOrder::new([SOURCE, PROFILE, OUTPUT])
            .keys()
            .map(FocusKey::as_str)
    }
}
