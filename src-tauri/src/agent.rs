use crate::board::{Checkers, Move};

pub mod mcts_hash;
pub mod mcts_tree;

pub trait Agent<C: Checkers> {
    fn get_best_move(&mut self, root: C) -> Move;
}