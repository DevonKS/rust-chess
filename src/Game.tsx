import { Map as IMap, List as IList } from 'immutable';
import React, { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";

import "./Game.css"

import Board from "./Board";
import SidePanel from "./SidePanel";

function mapGameState(gameState: any) {
  let pieceMapping = new IMap<string, string>(
    {
      "WhitePawn": "P",
      "WhiteRook": "R",
      "WhiteKnight": "N",
      "WhiteBishop": "B",
      "WhiteQueen": "Q",
      "WhiteKing": "K",
      "BlackPawn": "p",
      "BlackRook": "r",
      "BlackKnight": "n",
      "BlackBishop": "b",
      "BlackQueen": "q",
      "BlackKing": "k",
    }
  );
  let newState = {
    "gameId": gameState["game_id"],
    "pieces": gameState["pieces"].map(([p, s]) => [pieceMapping.get(p), s.toLowerCase()]),
    "validMoves": gameState["valid_moves"].reduce(
      (acc, [from, to]) => {
        from = from.toLowerCase();
        to = to.toLowerCase();
        if (!acc.has(from)) {
          return acc.set(from, IList<string>([to]));
        } else {
          if (!acc.get(from).includes(to)) {
            return acc.update(from, (l) => l.push(to));
          } else {
            return acc;
          }
        }
      },
      new IMap<string, string[]>(),
    ),
    "moves": gameState["moves"],
  };
  return newState;
}

function Game() {
  const [gameStates, setGameStates] = useState(new IList([{ "pieces": new IList<IList<string>>(), "moves": [], }]));
  const [moves, setMoves] = useState(new IList([[]]));
  const [currentGameState, setCurrentGameState] = useState(0);
  const [flipped, setFlipped] = useState(false);

  useEffect(() => {
    invoke('new_game').then((res) => {
      let newState = mapGameState(res);
      setGameStates(gameStates.push(newState));
      setMoves(moves.push([]));
      setCurrentGameState(currentGameState + 1);
    });
  }, []);

  // FIXME: need to handle promotions
  let makeMove = function(m: string) {
    invoke('make_move', { id: gameStates.last()["gameId"], m: m })
      .then((res) => {
        let newState = mapGameState(res);
        setGameStates(gameStates.push(newState));
        setMoves(moves.push([m.substring(0, 2), m.substring(2, 4)]));
        setCurrentGameState(currentGameState + 1);
      })
      .catch((error) => console.error(error));
  }

  return (
    <div className="game">
      <Board
        gameState={gameStates.get(currentGameState)}
        lastMove={moves.get(currentGameState)}
        makeMove={makeMove}
        canMove={currentGameState == gameStates.size - 1}
        flipped={flipped} />
      <SidePanel
        moves={gameStates.last().moves}
        currentGameState={currentGameState}
        setCurrentGameState={setCurrentGameState}
        numGameStates={gameStates.size}
        toggleFlipped={() => setFlipped(!flipped)} />
    </div>
  );
}

export default Game;
