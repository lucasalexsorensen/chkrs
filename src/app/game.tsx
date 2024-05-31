'use client';
import {
  invokeGetBestMove,
  invokeGetDefaultState,
  invokeGetLegalMoves,
  invokeMakeMove,
} from './api';
import { Player, BoardState, Move } from './types';
import { Fragment, useState, useEffect } from 'react';

type Index = number;
type Coords = [number, number];

const convertIndexToCoords = (i: Index): Coords => [Math.floor(i / 8), i % 8];
const convertCoordsToIndex = (coords: Coords): Index =>
  coords[0] * 8 + coords[1];

export function Game() {
  const [boardState, setBoardState] = useState<BoardState | null>(null);
  const [selectedIndex, setSelectedIndex] = useState<number | null>(null);
  const [possibleMoves, setPossibleMoves] = useState<Move[] | null>(null);

  useEffect(() => {
    const initialize = async () => {
      const defaultState = await invokeGetDefaultState();
      setBoardState(defaultState);
    };
    initialize().catch(console.error);
  }, []);

  useEffect(() => {
    const makeBestMove = async () => {
      if (boardState === null || boardState.turn !== 'Cpu') return;

      const bestMove = await invokeGetBestMove(boardState);
      const newBoardState = await invokeMakeMove(boardState, bestMove);
      setBoardState(newBoardState);
    };

    makeBestMove().catch(console.error);
  }, [boardState]);

  if (boardState === null) {
    return <div>Loading...</div>;
  }

  const handleClick = async (player: Player, index: number) => {
    if (player === 'Cpu') return;
    if (player !== boardState.turn) return;
    setSelectedIndex(index);

    const moves = (await invokeGetLegalMoves(boardState)).filter(
      (move) => convertCoordsToIndex(move.from) === index,
    );
    setPossibleMoves(moves);
  };

  const handleGhostClick = async (from: Coords, to: Coords) => {
    const move = { from, to, is_skip_move: Math.abs(from[0] - to[0]) === 2 };
    const newBoardState = await invokeMakeMove(boardState, move);
    setPossibleMoves(null);
    setBoardState(newBoardState);
  };

  function renderTileContents(index: Index) {
    if (boardState === null) return null;

    const [rowIdx, colIdx] = convertIndexToCoords(index);

    const tile = boardState.tiles[rowIdx][colIdx];

    if (tile !== null) {
      const player = tile.player;
      return Piece(
        player,
        tile.is_king,
        () => handleClick(player, index),
        selectedIndex === index,
      );
    }

    const ghostPiece = possibleMoves?.find(
      (move) => move.to[0] === rowIdx && move.to[1] === colIdx,
    );
    if (ghostPiece !== undefined) {
      const from = ghostPiece.from;
      return Piece(
        boardState.turn,
        false,
        () => handleGhostClick(from, [rowIdx, colIdx]),
        false,
        true,
      );
    }

    return null;
  }

  return (
    <Fragment>
      <div
        style={{ width: 480, height: 480 }}
        className="grid grid-cols-8 grid-rows-8 border-2 border-black"
      >
        {Array.from({ length: 64 }).map((_, i) => {
          return (
            <div
              key={i}
              className={`flex ${i % 2 === Math.floor(i / 8) % 2 ? 'bg-gray-900' : 'bg-red-900'}`}
            >
              {renderTileContents(i)}
            </div>
          );
        })}
      </div>
      <span className="mt-3 text-xl">
        <span
          className={` ${boardState.turn === 'Human' ? 'text-blue-500' : 'text-red-500'}`}
        >
          {boardState.turn}
        </span>
        &apos;s turn
      </span>
    </Fragment>
  );
}

function Piece(
  player: Player,
  isKing: boolean,
  onClickHandler: () => void,
  isSelected: boolean | null,
  isGhost = false,
) {
  return (
    <div
      onClick={onClickHandler}
      className={`m-auto h-5/6 w-10/12 cursor-pointer rounded-full border-2 ${player === 'Human' ? 'border-blue-400 bg-blue-500' : 'border-red-400 bg-red-500'} ${isKing ? 'ring-4 ring-yellow-400' : ''} ${isSelected ? 'ring-4 ring-green-400' : ''} ${isGhost ? 'opacity-50' : ''} `}
    />
  );
}
