pub(crate) mod game_over;
pub(super) mod high_score_entry;
pub(super) mod play_menu;
pub(super) mod score;
pub(super) mod text;

pub(super) trait DiscreteSelection {
    fn next(self) -> Self;
    fn previous(self) -> Self;
}
