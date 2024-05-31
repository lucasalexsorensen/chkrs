use crate::board::{Checkers, Move, Player};
use fnv::{FnvHashMap, FnvHashSet};
use itertools::Itertools;
use rand::prelude::*;
use rand::SeedableRng;
use rand::rngs::SmallRng;
use super::Agent;

#[derive(Default)]
pub struct Statistics {
    pub visits: usize,
    pub human_wins: usize,
    pub cpu_wins: usize,
}

const MAX_ROLLOUTS: usize = 500;


pub struct MctsHashAgent<C: Checkers> {
    stats: FnvHashMap<C, Statistics>,
    children: FnvHashMap<C, FnvHashSet<C>>,
    random: rand::rngs::SmallRng,
}

impl<C: Checkers> Default for MctsHashAgent<C> {
    fn default() -> Self {
        Self {
            stats: FnvHashMap::default(),
            children: FnvHashMap::default(),
            random: SmallRng::from_seed([6; 32]),
        }
    }
}

impl<C: Checkers> Agent<C> for MctsHashAgent<C> {
    fn get_best_move(&mut self, root: C) -> Move {
        self.stats.insert(root, Statistics::default());

        for _ in 0..20_000 {
            let path = self.select(root);
            let reward = self.rollout(*path.last().unwrap());
            self.backpropagate(path, reward);
        }

        let possible_moves = root.get_legal_moves();
        // just return max visit count
        let best_move = possible_moves.iter().max_by_key(|mv| {
            let child = root.make_move(**mv);
            self.stats.get(&child).unwrap().visits
        }).unwrap();
        *best_move
    }
}


impl<C: Checkers> MctsHashAgent<C> {
    fn select(&mut self, root: C) -> Vec<C> {
        let mut path = vec![root];
        loop {
            let node = *path.last().unwrap();
            if node.is_game_over() {
                break;
            }
            
            if !self.stats.contains_key(&node) {
                self.stats.insert(node, Statistics { visits: 0, human_wins: 0, cpu_wins: 0 });
            }
            if !self.children.contains_key(&node) {
                let children = node.get_legal_moves().iter().map(|mv| node.make_move(*mv)).collect();
                self.children.insert(node, children);
            }
            let children = self.children.get(&node).unwrap();
            let unvisited_children = children.iter().filter(|child| !self.stats.contains_key(child));

            // if there are unvisited children, select one and return immediately
            if let Some(unvisited_child) = unvisited_children.last() {
                self.stats.insert(*unvisited_child, Statistics::default());
                path.push(*unvisited_child);
                return path;
            }

            // select by max UCT
            let best_child_idx = children.iter().map(|child| {
                let parent_stats = self.stats.get(&node).unwrap();
                let child_stats = self.stats.get(child).unwrap();

                let (wins, losses) = match node.get_turn() {
                    Player::Human => (child_stats.human_wins, child_stats.cpu_wins),
                    Player::Cpu => (child_stats.cpu_wins, child_stats.human_wins),
                };
                let exploitation = (wins as f64 - losses as f64) / child_stats.visits as f64;
                let exploration = (2.0f64 * (parent_stats.visits as f64).ln() / child_stats.visits as f64).sqrt();
                let uct = exploitation + 1.41 * exploration;
                uct
            }).position_max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();

            path.push(*children.iter().nth(best_child_idx).unwrap());
        }

        path
    }

    fn rollout(&mut self, state: C) -> Option<Player> {
        let mut rollout_state = state.clone();
        let mut counter = 0;
        while !rollout_state.is_game_over() {
            let possible_moves = rollout_state.get_legal_moves();
            if possible_moves.is_empty() {
                return None;
            }
            let mv = possible_moves.choose(&mut self.random).unwrap();
            rollout_state = rollout_state.make_move(*mv);
            counter += 1;

            if counter > MAX_ROLLOUTS {
                return None;
            }
        }
        rollout_state.get_winner()
    }

    fn backpropagate(&mut self, path: Vec<C>, result: Option<Player>) {
        for node in path {
            let stats = self.stats.get_mut(&node).unwrap();
            stats.visits += 1;
            match result {
                Some(Player::Human) => stats.human_wins += 1,
                Some(Player::Cpu) => stats.cpu_wins += 1,
                _ => (),
            }
        }
    }
}