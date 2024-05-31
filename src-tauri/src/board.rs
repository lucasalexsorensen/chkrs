use ts_rs::TS;

pub mod fast;
pub mod slow;

use super::public;

#[derive(TS, Debug, PartialEq, Eq, Clone, Copy, Hash, serde::Deserialize, serde::Serialize)]
pub enum Player {
    Human,
    Cpu,
}

#[derive(TS, Debug, PartialEq, Eq, Clone, Copy, Hash, serde::Deserialize, serde::Serialize)]
#[ts(export)]
pub struct Move {
    pub from: (u8, u8),
    pub to: (u8, u8),
    pub is_skip_move: bool,
}

impl std::ops::Not for Player {
    type Output = Self;

    fn not(self) -> Self {
        match self {
            Player::Human => Player::Cpu,
            Player::Cpu => Player::Human,
        }
    }
}

pub trait Checkers: std::hash::Hash + PartialEq + Eq + Clone + Copy + From<public::BoardState> + Into<public::BoardState> {
    fn default() -> Self;
    fn is_game_over(&self) -> bool;
    fn get_winner(&self) -> Option<Player>;
    fn get_legal_moves(&self) -> Vec<Move>;
    fn make_move(&self, mv: Move) -> Self;
    fn get_turn(&self) -> Player;
}
