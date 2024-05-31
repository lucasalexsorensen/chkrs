use crate::agent::Agent;
use crate::board::Checkers;
use crate::public;
use crate::board;
use crate::agent;


#[tauri::command]
pub async fn get_legal_moves(state: public::BoardState) -> Result<Vec<board::Move>, ()> {
    let state: board::slow::BoardState = state.into();
    let moves = state.get_legal_moves();
    let result = moves.iter().map(|m| (*m).into()).collect();

    Ok(result)
}

#[tauri::command]
pub async fn make_move(state: public::BoardState, mv: board::Move) -> Result<public::BoardState, ()> {
    let mut state: board::slow::BoardState = state.into();
    state = state.make_move(mv.into());
    Ok(state.into())
}

#[tauri::command]
pub async fn get_default_state() -> Result<public::BoardState, ()> {
    let state = board::slow::BoardState::default();
    Ok(state.into())
}

#[tauri::command]
pub async fn get_best_move(state: public::BoardState) -> Result<board::Move, ()> {
    let mut agent = agent::mcts_hash::MctsHashAgent::default();
    let state: board::slow::BoardState = state.into();
    println!("Getting best move");
    let mv = agent.get_best_move(state);
    println!("Got best move {:?}", mv);
    Ok(mv.into())
}