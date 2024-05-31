use ts_rs::TS;
use crate::board::{Player, Move};

#[derive(TS, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Copy)]
pub struct Tile {
    pub player: Player,
    pub is_king: bool
}


#[derive(TS, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone)]
#[ts(export)]
pub struct BoardState {
    pub tiles: [[Option<Tile>; 8]; 8],
    pub turn: Player
}
