use iced::event;
use iced::keyboard::{Key, Modifiers, key::Named};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyboardCommand {
    FocusNext,
    FocusPrevious,
    ActivatePrimary,
    DismissTransient,
}

#[must_use]
pub fn keyboard_command(
    key: &Key,
    modifiers: Modifiers,
    repeat: bool,
    status: event::Status,
) -> Option<KeyboardCommand> {
    if repeat || status == event::Status::Captured {
        return None;
    }

    match key {
        Key::Named(Named::Tab) if modifiers.shift() => Some(KeyboardCommand::FocusPrevious),
        Key::Named(Named::Tab) => Some(KeyboardCommand::FocusNext),
        Key::Named(Named::Enter) => Some(KeyboardCommand::ActivatePrimary),
        Key::Named(Named::Escape) => Some(KeyboardCommand::DismissTransient),
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
            Some(KeyboardCommand::ActivatePrimary)
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
    }

    fn order_keys() -> [&'static str; 3] {
        FocusOrder::new([SOURCE, PROFILE, OUTPUT])
            .keys()
            .map(FocusKey::as_str)
    }
}
