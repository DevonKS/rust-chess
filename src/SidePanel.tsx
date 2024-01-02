import { LuChevronFirst, LuChevronLeft, LuChevronRight, LuChevronLast, LuRepeat2 } from "react-icons/lu";

import './SidePanel.css'

type SidePanelProps = {
  moves: string[];
  currentGameState: number,
  setCurrentGameState: Function,
  numGameStates: number,
  toggleFlipped: Function,
}

function halfMove(currentGameState: number, numGameStates: number, numMoves: number): number {
  let offset = numGameStates - numMoves;
  let halfMove = Math.max(currentGameState - offset + 1, 0);
  return halfMove;
}

function currentGameState(halfMove: number, numGameStates: number, numMoves: number): number {
  let offset = numGameStates - numMoves;
  return halfMove + offset - 1;
}

function Move(moveNum: number, moveA: string, moveB: string, currentHalfMove: number, numGameStates: number, numMoves: number, setCurrentGameState: Function) {
  let moveAMoveNumber = (moveNum * 2) - 1;
  let moveBMoveNumber = moveNum * 2;
  let moveAClass = moveAMoveNumber === currentHalfMove ? "current-move" : "";
  let moveBClass = moveBMoveNumber === currentHalfMove ? "current-move" : "";
  let moveAClick = moveAMoveNumber !== currentHalfMove ? () => setCurrentGameState(currentGameState(moveAMoveNumber, numGameStates, numMoves)) : undefined;
  let moveBClick = moveBMoveNumber !== currentHalfMove ? () => setCurrentGameState(currentGameState(moveBMoveNumber, numGameStates, numMoves)) : undefined;
  // FIXME: Change cursor over the move divs
  return (
    <div key={moveNum} className="move">
      {`${moveNum}.`}
      <div className={moveAClass} onClick={moveAClick}>{moveA}</div>
      <div className={moveBClass} onClick={moveBClick}>{moveB}</div>
    </div>
  )
}

function SidePanel(props: SidePanelProps) {
  let currentGameState = props.currentGameState;
  let numGameStates = props.numGameStates;
  let numMoves = props.moves.length;

  // FIXME: give this a proper type
  var x: string[][] = [];
  for (var i = 0; i < props.moves.length; i += 2) {
    let tmp = [];
    tmp.push(i / 2 + 1);
    tmp.push(props.moves[i]);

    if (i + 1 < props.moves.length) {
      tmp.push(props.moves[i + 1]);
    }
    x.push(tmp);
  }

  let currentHalfMove = halfMove(currentGameState, numGameStates, numMoves)

  // FIXME: Make the buttons look nice
  return (
    <div className="side-panel">

      <div className="moves">
        {x.map((i) => Move(i[0], i[1], i[2], currentHalfMove, numGameStates, numMoves, props.setCurrentGameState))}
      </div>
      <div className="controls">
        <button onClick={() => props.setCurrentGameState(1)}><LuChevronFirst size="2em" style={{ verticalAlign: 'middle' }} /></button>
        <button onClick={() => {
          if (currentGameState > 1) {
            props.setCurrentGameState(currentGameState - 1);
          }
        }}><LuChevronLeft size="2em" style={{ verticalAlign: 'middle' }} /></button>
        <button onClick={() => {
          if (currentGameState < numGameStates - 1) {
            props.setCurrentGameState(currentGameState + 1);
          }
        }}><LuChevronRight size="2em" style={{ verticalAlign: 'middle' }} /></button>
        <button onClick={() => props.setCurrentGameState(numGameStates - 1)}><LuChevronLast size="2em" style={{ verticalAlign: 'middle' }} /></button>
        <button onClick={() => props.toggleFlipped()}><LuRepeat2 size="2em" style={{ verticalAlign: 'middle' }} /></button>
      </div>
    </div >
  );
}

export default SidePanel;
