use crate::player::Player;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FinalState {
    Win(Player),
    Draw,
}
