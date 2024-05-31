// use crate::board;
// use crate::public;
// use board::BitSet;


// // board::Player <-> public::Player conversions
// impl Into<board::Player> for public::Player {
//     fn into(self) -> board::Player {
//         match self {
//             public::Player::Human => board::Player::Human,
//             public::Player::Cpu => board::Player::Cpu
//         }
//     }
// }

// impl Into<public::Player> for board::Player {
//     fn into(self) -> public::Player {
//         match self {
//             board::Player::Human => public::Player::Human,
//             board::Player::Cpu => public::Player::Cpu
//         }
//     }
// }

// // board::BoardState <-> public::BoardState conversions
// impl Into<board::BoardState> for public::BoardState {
//     fn into(self) -> board::BoardState {
//         let mut tiles_human: u64 = 0;
//         let mut tiles_cpu = 0;
//         let mut kings = 0;
//         self.tiles.iter().enumerate().for_each(|(i,player)| {
//             match player {
//                 Some(public::Player::Human) => { tiles_human = tiles_human.set_at_position(i, true); },
//                 Some(public::Player::Cpu) => { tiles_cpu = tiles_cpu.set_at_position(i, true); },
//                 _ => (),
//             };
//         });

//         board::BoardState { tiles_human, tiles_cpu, kings, turn: self.turn.into() }
//     }
// }

// impl Into<public::BoardState> for board::BoardState {
//     fn into(self) -> public::BoardState {
//         let mut tiles = vec![None; 35];
//         self.tiles_human.iter_ones().for_each(|i| {
//             tiles[i as usize] = Some(public::Player::Human);
//         });
//         self.tiles_cpu.iter_ones().for_each(|i| {
//             tiles[i as usize] = Some(public::Player::Cpu);
//         });
//         public::BoardState { tiles, turn: self.turn.into() }

//     }
// }


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_player_conversion() {
//         assert_eq!(board::Player::Human, public::Player::Human.into());
//         assert_eq!(board::Player::Cpu, public::Player::Cpu.into());
//         assert_eq!(public::Player::Human, board::Player::Human.into());
//         assert_eq!(public::Player::Cpu, board::Player::Cpu.into());
//     }

//     #[test]
//     fn test_move_conversion() {
//         let m = board::Move { position: 9, direction: board::Direction::DownRight, skip: false };
//         let p = public::Move { from: 9, to: 14 };
//         assert_eq!(m, p.into());
//         assert_eq!(p, m.into());

//         let m = board::Move { position: 19, direction: board::Direction::UpLeft, skip: true };
//         let p = public::Move { from: 19, to: 9 };
//         assert_eq!(m, p.into());
//         assert_eq!(p, m.into());
//     }

//     #[test]
//     fn test_board_state_conversion() {
//         // check "identity" conversion
//         let b = board::BoardState::default();
//         let p: public::BoardState = b.into();
        
//         println!("{:?}", b);
//         println!("{:?}", p);

//         assert_eq!(b, p.into());
//     }

// }