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
          return acc.update(from, (l) => l.push(to));
        }
      },
      new IMap<string, string[]>(),
    ),
    "moves": gameState["moves"],
  };
  return newState;
}

function Game() {
  const [gameState, setGameState] = useState({ "pieces": new IList<IList<string>>(), "moves": [], });

  useEffect(() => {
    invoke('new_game').then((res) => {
      let newState = mapGameState(res);
      setGameState(newState);
    });
  }, []);

  console.log(gameState.moves);

  // FIXME: need to handle promotions
  let makeMove = function(m: string) {
    invoke('make_move', { id: gameState["gameId"], m: m })
      .then((res) => {
        setGameState(mapGameState(res));
      })
      .catch((error) => console.error(error));
  }

  return (
    <div className="game">
      <Board gameState={gameState} makeMove={makeMove} />
      <SidePanel moves={gameState.moves} />
    </div>
  );
}

export default Game;
