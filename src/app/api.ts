import { invoke } from '@tauri-apps/api/tauri';
import { BoardState, Move } from './types';

export async function invokeGetDefaultState(): Promise<BoardState> {
  return invoke<BoardState>('get_default_state');
}

export async function invokeGetLegalMoves(state: BoardState): Promise<Move[]> {
  return invoke<Move[]>('get_legal_moves', { state });
}

export async function invokeMakeMove(
  state: BoardState,
  mv: Move,
): Promise<BoardState> {
  return invoke<BoardState>('make_move', { state, mv });
}

export async function invokeGetBestMove(state: BoardState): Promise<Move> {
  return invoke<Move>('get_best_move', { state });
}
