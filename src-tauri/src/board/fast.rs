//    -------CPU--------
//  -1|--00--01--02--03|
//    |04--05--06--07--|08
//  08|--09--10--11--12|
//    |13--14--15--16--|17
//  17|--18--19--20--21|
//    |22--23--24--25--|26
//  26|--27--28--29--30|
//    |31--32--33--34--|35
//    -------HUMAN------
// "dead" positions: [-1, 08, 17, 26, 35]
// I found this blog post which does sort of the same thing: https://3dkingdoms.com/checkers/bitboards.htm
use super::{Checkers, Player};


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Direction {
    UpLeft,
    UpRight,
    DownLeft,
    DownRight
}


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Move {
    pub position: usize,
    pub direction: Direction,
    pub skip: bool
}



const LEGAL_TILES_MASK: u64 = 0b0000000000000000000000000000011111111011111111011111111011111111;

const fn tiles_for_player(player: Player) -> u64 {
    match player {
        Player::Human => 0b0000000000000000000000000000011111111011110000000000000000000000,
        Player::Cpu =>   0b0000000000000000000000000000000000000000000000000001111011111111,
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct BoardState {
    pub tiles_human: u64,
    pub tiles_cpu: u64,
    pub kings: u64,
    pub turn: Player
}

impl Default for BoardState {
    fn default() -> Self {
        BoardState {
            tiles_human: tiles_for_player(Player::Human),
            tiles_cpu: tiles_for_player(Player::Cpu),
            kings: 0,
            turn: Player::Human
        }
    }
}

pub trait BitSet {
    fn iter_ones(&self) -> impl Iterator<Item = usize>;
    fn iter_zeros(&self) -> impl Iterator<Item = usize>;
    fn get_at_position(&self, position: usize) -> Option<bool>;
    fn set_at_position(&self, position: usize, value: bool) -> Self;
}

impl BitSet for u64 {
    fn iter_ones(&self) -> impl Iterator<Item = usize> {
        (0..36 as usize).filter(move |i| self & (1 << i) != 0)
    }
    
    fn iter_zeros(&self) -> impl Iterator<Item = usize> {
        (0..36 as usize).filter(move |i| self & (1 << i) == 0)
    }

    fn get_at_position(&self, position: usize) -> Option<bool> {
        // if position is illegal, return None
        if !LEGAL_TILES_MASK & (1 << position) != 0 {
            return None;
        }
        Some(self & (1 << position) != 0)
    }

    fn set_at_position(&self, position: usize, value: bool) -> Self {
        if value {
            self | (1 << position)
        } else {
            self & !(1 << position)
        }
    }
}

pub const fn delta_for_dir(dir: &Direction) -> i8 {
    match dir {
        Direction::UpLeft => -5,
        Direction::UpRight => -4,
        Direction::DownLeft => 4,
        Direction::DownRight => 5
    }
}

pub const fn dir_for_delta(delta: i8) -> Direction {
    match delta {
        -5 => Direction::UpLeft,
        -4 => Direction::UpRight,
        4 => Direction::DownLeft,
        5 => Direction::DownRight,
        -10 => Direction::UpLeft,
        -8 => Direction::UpRight,
        8 => Direction::DownLeft,
        10 => Direction::DownRight,
        _ => panic!("Invalid delta")
    }
}

impl BoardState {
    pub fn get_winner(&self) -> Option<Player> {
        if self.tiles_human == 0 {
            Some(Player::Cpu)
        } else if self.tiles_cpu == 0 {
            Some(Player::Human)
        } else {
            None
        }
    }

    pub fn is_game_over(&self) -> bool {
        self.get_winner().is_some()
    }

    pub fn get_legal_moves(&self) -> Vec<Move> {
        // let vacant_tiles = !(self.tiles_human | self.tiles_cpu) & LEGAL_TILES_MASK;
        let (own_tiles, enemy_tiles) = match self.turn {
            Player::Human => (self.tiles_human, self.tiles_cpu),
            Player::Cpu => (self.tiles_cpu, self.tiles_human)
        };

        let moves = own_tiles.iter_ones().flat_map(|pos| {
            let is_king = self.kings.get_at_position(pos).unwrap();

            let directions = if is_king {
                vec![Direction::UpLeft, Direction::UpRight, Direction::DownLeft, Direction::DownRight]
            } else {
                match self.turn {
                    Player::Human => vec![Direction::UpLeft, Direction::UpRight],
                    Player::Cpu => vec![Direction::DownLeft, Direction::DownRight]
                }
            };

            directions.iter().map(|dir| {
                let next_pos = (pos as i8 + delta_for_dir(&dir)) as usize;
                match (own_tiles.get_at_position(next_pos), enemy_tiles.get_at_position(next_pos)) {
                    (Some(false), Some(true)) => {
                        let next_next_pos = (next_pos as i8 + delta_for_dir(&dir)) as usize;
                        match (own_tiles.get_at_position(next_next_pos), enemy_tiles.get_at_position(next_next_pos)) {
                            (Some(false), Some(false)) => Some(Move { position: pos, direction: *dir, skip: true }),
                            _ => None
                        }
                    },
                    (Some(false), Some(false)) => Some(Move { position: pos, direction: *dir, skip: false }),
                    _ => None
                }
            }).collect::<Vec<_>>()
        }).filter_map(|x| x).collect::<Vec<_>>();

        // if there is a move with skip, only return the moves with skip
        // if there is no move with skip, return all
        if moves.iter().any(|mv| mv.skip) {
            moves.into_iter().filter(|mv| mv.skip).collect()
        } else {
            moves
        }
    }

    pub fn make_move(&self, mv: Move) -> Self {
        let (own_tiles, enemy_tiles) = match self.turn {
            Player::Human => (self.tiles_human, self.tiles_cpu),
            Player::Cpu => (self.tiles_cpu, self.tiles_human)
        };

        let is_king = self.kings.get_at_position(mv.position).unwrap();
        let next_pos = (mv.position as i8 + delta_for_dir(&mv.direction)) as usize;
        
        let (new_own_tiles, new_enemy_tiles, new_kings) = match mv.skip {
            true => {
                let next_next_pos = (next_pos as i8 + delta_for_dir(&mv.direction)) as usize;
                let new_enemy_tiles = enemy_tiles.set_at_position(next_pos, false);
                let new_own_tiles = own_tiles.set_at_position(next_next_pos, true).set_at_position(mv.position, false);

                let mut new_kings = self.kings.set_at_position(next_pos, false);
                if is_king {
                    new_kings = self.kings.set_at_position(mv.position, false).set_at_position(next_next_pos, true);
                }

                if next_next_pos < 4 || next_next_pos >= 31 {
                    new_kings = new_kings.set_at_position(next_next_pos, true);
                }

                (new_own_tiles, new_enemy_tiles, new_kings)
            },
            false => {
                let new_own_tiles = own_tiles.set_at_position(mv.position, false).set_at_position(next_pos, true);
                let mut new_kings = self.kings;

                if is_king {
                    new_kings = new_kings.set_at_position(mv.position, false).set_at_position(next_pos, true);
                }
                
                if next_pos < 4 || next_pos >= 31 {
                    new_kings = new_kings.set_at_position(next_pos, true);
                }

                (new_own_tiles, enemy_tiles, new_kings)
            }
        };


        let mut new_state = Self {
            tiles_human: match self.turn {
                Player::Human => new_own_tiles,
                Player::Cpu => new_enemy_tiles,
            },
            tiles_cpu: match self.turn {
                Player::Human => new_enemy_tiles,
                Player::Cpu => new_own_tiles,
            },
            kings: new_kings,
            turn: self.turn
        };

        // if there are still more skip moves, keep the turn
        if mv.skip && new_state.get_legal_moves().iter().any(|mv| mv.skip) {
            new_state.turn = self.turn;
        } else {
            new_state.turn = !self.turn;
        }

        new_state
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skip_moves() {
        let mut state = BoardState::default();
        state.tiles_human = 1 << 24;
        state.tiles_cpu = 1 << 20;
        // should skip like 24 -> 20 -> 16
        // additionally, the skip is mandatory, which means there is only one legal move
        let legal_moves = state.get_legal_moves();
        assert_eq!(legal_moves, vec![Move { position: 24, direction: Direction::UpRight, skip: true }]);
    }

    #[test]
    fn test_moves() {
        let mut state = BoardState::default();
        let mut legal_moves = state.get_legal_moves();
        legal_moves.sort();
        let mut target_moves = vec![
            Move { position: 22, direction: Direction::UpRight, skip: false },
            Move { position: 23, direction: Direction::UpRight, skip: false },
            Move { position: 23, direction: Direction::UpLeft, skip: false },
            Move { position: 24, direction: Direction::UpRight, skip: false },
            Move { position: 24, direction: Direction::UpLeft, skip: false },
            Move { position: 25, direction: Direction::UpRight, skip: false },
            Move { position: 25, direction: Direction::UpLeft, skip: false },
        ];
        target_moves.sort();
        assert_eq!(legal_moves, target_moves);

        state.turn = Player::Cpu;
        legal_moves = state.get_legal_moves();
        legal_moves.sort();
        target_moves = vec![
            Move { position: 10, direction: Direction::DownLeft, skip: false },
            Move { position: 10, direction: Direction::DownRight, skip: false },
            Move { position: 11, direction: Direction::DownLeft, skip: false },
            Move { position: 11, direction: Direction::DownRight, skip: false },
            Move { position: 12, direction: Direction::DownLeft, skip: false },
            Move { position: 12, direction: Direction::DownRight, skip: false },
            Move { position: 13, direction: Direction::DownLeft, skip: false },  
        ];
        target_moves.sort();

    }

    #[test]
    fn test_make_move() {
        let mut state = BoardState::default();
        state = state.make_move(Move { position: 22, direction: Direction::UpRight, skip: false });
        state = state.make_move(Move { position: 12, direction: Direction::DownLeft, skip: false });
        
        let mut target_moves = vec![
            Move { position: 18, direction: Direction::UpLeft, skip: false },
            Move { position: 18, direction: Direction::UpRight, skip: false },
            Move { position: 23, direction: Direction::UpRight, skip: false },
            Move { position: 24, direction: Direction::UpRight, skip: false },
            Move { position: 24, direction: Direction::UpLeft, skip: false },
            Move { position: 25, direction: Direction::UpRight, skip: false },
            Move { position: 25, direction: Direction::UpLeft, skip: false },
            Move { position: 27, direction: Direction::UpLeft, skip: false },
        ];
        target_moves.sort();

        assert_eq!(
            target_moves,
            state.get_legal_moves()
        )
    }

    #[test]
    fn test_bitsets() {
        assert_eq!("0000000000000000000000000000011111111011111111011111111011111111", format!("{:064b}", LEGAL_TILES_MASK));
        assert_eq!("1111111101111111101111111101111111100000000000000000000000000000", format!("{:064b}", LEGAL_TILES_MASK.reverse_bits()));
        
        let valid_indices = vec![0, 1, 2, 3, 4, 5, 6, 7, 9, 10, 11, 12, 13, 14, 15, 16, 18, 19, 20, 21, 22, 23, 24, 25, 27, 28, 29, 30, 31, 32, 33, 34];

        assert_eq!(valid_indices, LEGAL_TILES_MASK.iter_ones().collect::<Vec<usize>>());
    }
}