use crate::domain::MirrorPolicy;
use crate::i18n::UiDirection;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogicalEdge {
    Start,
    End,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalEdge {
    Left,
    Right,
}

#[must_use]
pub const fn physical_edge(direction: UiDirection, edge: LogicalEdge) -> PhysicalEdge {
    match (direction, edge) {
        (UiDirection::Ltr, LogicalEdge::Start) | (UiDirection::Rtl, LogicalEdge::End) => {
            PhysicalEdge::Left
        }
        (UiDirection::Ltr, LogicalEdge::End) | (UiDirection::Rtl, LogicalEdge::Start) => {
            PhysicalEdge::Right
        }
    }
}

#[must_use]
pub const fn should_mirror(direction: UiDirection, policy: MirrorPolicy) -> bool {
    match policy {
        MirrorPolicy::FollowLocale => matches!(direction, UiDirection::Rtl),
        MirrorPolicy::Never => false,
        MirrorPolicy::Always => true,
    }
}

#[must_use]
pub const fn logical_pair<T>(direction: UiDirection, start: T, end: T) -> [T; 2] {
    match direction {
        UiDirection::Ltr => [start, end],
        UiDirection::Rtl => [end, start],
    }
}

#[must_use]
pub fn logical_sequence<T>(direction: UiDirection, mut items: Vec<T>) -> Vec<T> {
    if matches!(direction, UiDirection::Rtl) {
        items.reverse();
    }
    items
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn logical_edges_map_to_physical_edges_by_locale_direction() {
        assert_eq!(
            physical_edge(UiDirection::Ltr, LogicalEdge::Start),
            PhysicalEdge::Left
        );
        assert_eq!(
            physical_edge(UiDirection::Ltr, LogicalEdge::End),
            PhysicalEdge::Right
        );
        assert_eq!(
            physical_edge(UiDirection::Rtl, LogicalEdge::Start),
            PhysicalEdge::Right
        );
        assert_eq!(
            physical_edge(UiDirection::Rtl, LogicalEdge::End),
            PhysicalEdge::Left
        );
    }

    #[test]
    fn logical_pair_reorders_visual_children_without_changing_semantics() {
        assert_eq!(
            logical_pair(UiDirection::Ltr, "start", "end"),
            ["start", "end"]
        );
        assert_eq!(
            logical_pair(UiDirection::Rtl, "start", "end"),
            ["end", "start"]
        );
    }

    #[test]
    fn logical_sequence_reverses_visual_order_only_for_rtl() {
        assert_eq!(
            logical_sequence(UiDirection::Ltr, vec![1, 2, 3]),
            vec![1, 2, 3]
        );
        assert_eq!(
            logical_sequence(UiDirection::Rtl, vec![1, 2, 3]),
            vec![3, 2, 1]
        );
    }

    #[test]
    fn mirror_policy_is_explicit() {
        assert!(should_mirror(UiDirection::Rtl, MirrorPolicy::FollowLocale));
        assert!(!should_mirror(UiDirection::Ltr, MirrorPolicy::FollowLocale));
        assert!(!should_mirror(UiDirection::Rtl, MirrorPolicy::Never));
        assert!(should_mirror(UiDirection::Ltr, MirrorPolicy::Always));
    }
}
