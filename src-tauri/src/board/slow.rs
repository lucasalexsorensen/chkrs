// this is the less efficient implementation of the checkers board
// it does not use bitboards and is not optimized for speed
use super::{Checkers, Player, Move};
use super::super::public;


// #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
// pub struct Move {
//     pub from: (u8, u8),
//     pub to: (u8, u8),
//     pub is_skip_move: bool
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tile {
    pub player: Player,
    pub is_king: bool
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct BoardState {
    tiles: [[Option<Tile>; 8]; 8],
    turn: Player
}

impl From<public::Tile> for Tile {
    fn from(value: public::Tile) -> Self {
        Self {
            player: value.player,
            is_king: value.is_king
        }
    }
}

impl From<Tile> for public::Tile {
    fn from(value: Tile) -> Self {
        Self {
            player: value.player,
            is_king: value.is_king
        }
    }
}

// conversion logic
impl From<public::BoardState> for BoardState {
    fn from(value: public::BoardState) -> Self {
        BoardState {
            tiles: value.tiles.map(|row| row.map(|tile| tile.map(|tile| tile.into()))),
            turn: value.turn
        }
    }
}

impl From<BoardState> for public::BoardState {
    fn from(value: BoardState) -> Self {
        public::BoardState {
            tiles: value.tiles.map(|row| row.map(|tile| tile.map(|tile| tile.into()))),
            turn: value.turn
        }
    }
}

// impl From<public::Move> for Move {
//     fn from(value: public::Move) -> Self {
//         let diff = (value.to.0 as i8 - value.from.0 as i8, value.to.1 as i8 - value.from.1 as i8);
//         Self {
//             from: (value.from.0, value.from.1),
//             to: (value.to.0, value.to.1),
//             is_skip_move: diff.0.abs() > 1
//         }
//     }
// }

// impl From<Move> for public::Move {
//     fn from(value: Move) -> Self {
//         Self {
//             from: (value.from.0, value.from.1),
//             to: (value.to.0, value.to.1),
//         }
//     }
// }


impl Checkers for BoardState {
    fn default() -> Self {
        let mut tiles = [[None; 8]; 8];

        for j in 0..8 {
            for i in 0..3 {
                if (i + j) % 2 == 1 {
                    tiles[i][j] = Some(Tile {
                        player: Player::Cpu,
                        is_king: false
                    });
                }
            }
            for i in 5..8 {
                if (i + j) % 2 == 1 {
                    tiles[i][j] = Some(Tile {
                        player: Player::Human,
                        is_king: false
                    });
                }
            }
        }

        Self {
            tiles,
            turn: Player::Human
        }
    }

    fn get_turn(&self) -> Player {
        self.turn
    }

    fn is_game_over(&self) -> bool {
        let has_human_tiles = self.tiles.iter().flatten().any(|tile| {
            if let Some(tile) = tile {
                tile.player == Player::Human
            } else {
                false
            }
        });

        let has_cpu_tiles = self.tiles.iter().flatten().any(|tile| {
            if let Some(tile) = tile {
                tile.player == Player::Cpu
            } else {
                false
            }
        });

        !has_human_tiles || !has_cpu_tiles
    }

    fn get_winner(&self) -> Option<Player> {
        let first_valid_tile = self.tiles.iter().flatten().find(|tile| tile.is_some()).unwrap();
        Some(first_valid_tile.unwrap().player)
    }

    fn get_legal_moves(&self) -> Vec<Move> {
        let own_tiles = self.tiles.iter().flatten().enumerate().filter(|(i,tile)| match tile {
            Some(tile) if tile.player == self.turn => true,
            _ => false
        });
        
        let mut moves = own_tiles.flat_map(|(i, tile)| {
            let tile = tile.unwrap();
            let (row, col) = (i / 8, i % 8);
            let mut moves = vec![];

            // cpu moves down (i.e. increasing row)
            // human moves up (i.e. decreasing row)
            // kings can move in both directions
            let row_dir = match (tile.player, tile.is_king) {
                (_, true) => vec![-1, 1],
                (Player::Cpu, false) => vec![1],
                (Player::Human, false) => vec![-1]
            };

            for row_offset in row_dir {
                for col_offset in vec![-1, 1] {
                    let new_row = row as i8 + row_offset;
                    let new_col = col as i8 + col_offset;

                    // if the new position is out of bounds, skip
                    if new_row < 0 || new_row >= 8 || new_col < 0 || new_col >= 8 {
                        continue;
                    }

                    let next_tile = self.tiles[new_row as usize][new_col as usize];
                    match next_tile {
                        Some(next_tile) => if next_tile.player != self.turn {
                            // if the new position is occupied by enemy, check if the next position is empty
                            let next_next_row = new_row + row_offset;
                            let next_next_col = new_col + col_offset;
                            if next_next_row < 0 || next_next_row >= 8 || next_next_col < 0 || next_next_col >= 8 {
                                continue;
                            }
                            let next_next_tile = self.tiles[next_next_row as usize][next_next_col as usize];
                            if next_next_tile.is_none() {
                                moves.push(Move {
                                    from: (row as u8, col as u8),
                                    to: (next_next_row as u8, next_next_col as u8),
                                    is_skip_move: true
                                });
                            }
                        },
                        None => {
                            moves.push(Move {
                                from: (row as u8, col as u8),
                                to: (new_row as u8, new_col as u8),
                                is_skip_move: false
                            });
                        }
                    }
                }
            }

            

            moves
        }).collect::<Vec<Move>>();

        // if moves contain any skip moves, filter out non-skip moves
        if moves.iter().any(|m| m.is_skip_move) {
            moves.retain(|m| m.is_skip_move);
        }
        moves
    }

    fn make_move(&self, mv: Move) -> Self {
        let mut new_tiles = self.tiles.clone();
        let (from_row, from_col) = (mv.from.0 as usize, mv.from.1 as usize);
        let (to_row, to_col) = (mv.to.0 as usize, mv.to.1 as usize);

        new_tiles[to_row][to_col] = new_tiles[from_row][from_col];
        new_tiles[from_row][from_col] = None;

        // check for king promotion
        if let Some(tile) = new_tiles[to_row][to_col] {
            if (tile.player == Player::Cpu && to_row == 7) || (tile.player == Player::Human && to_row == 0) {
                new_tiles[to_row][to_col] = Some(Tile {
                    player: tile.player,
                    is_king: true
                });
            }
        }

        if mv.is_skip_move {
            let skip_row = (from_row + to_row) / 2;
            let skip_col = (from_col + to_col) / 2;
            new_tiles[skip_row][skip_col] = None;
        }

        let mut next_state = Self {
            tiles: new_tiles,
            turn: self.turn
        };

        if !mv.is_skip_move || !next_state.get_legal_moves().iter().any(|m| m.is_skip_move) {
            next_state.turn = !self.turn;
        }
            
        next_state

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let board = BoardState::default();
        assert_eq!(board.tiles[0][1].unwrap().player, Player::Cpu);
        assert_eq!(board.tiles[7][0].unwrap().player, Player::Human);
    }

    #[test]
    fn test_get_legal_moves() {
        let board = BoardState::default();
        let moves = board.get_legal_moves();
        assert_eq!(moves.len(), 7);
    }

    #[test]
    fn test_is_game_over() {
        let mut board = BoardState::default();
        assert_eq!(board.is_game_over(), false);
        board.tiles = [[None; 8]; 8];
        assert_eq!(board.is_game_over(), true);
    }
}