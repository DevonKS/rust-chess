import { Map as IMap, Set as ISet, List as IList, is } from 'immutable';
import React, { useState, useEffect } from "react";
import Modal from 'react-modal';
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./Board.css";

const colToFile = new IMap<number, string>([[0, "a"], [1, "b"], [2, "c"], [3, "d"], [4, "e"], [5, "f"], [6, "g"], [7, "h"]]);
const rowToRank = new IMap<number, string>([[0, "8"], [1, "7"], [2, "6"], [3, "5"], [4, "4"], [5, "3"], [6, "2"], [7, "1"]]);
const xToFile = new IMap<number, string>([[0, "a"], [100, "b"], [200, "c"], [300, "d"], [400, "e"], [500, "f"], [600, "g"], [700, "h"]]);
const yToRank = new IMap<number, string>([[700, "1"], [600, "2"], [500, "3"], [400, "4"], [300, "5"], [200, "6"], [100, "7"], [0, "8"]]);
const fileToX = new IMap<string, number>([["a", 0], ["b", 100], ["c", 200], ["d", 300], ["e", 400], ["f", 500], ["g", 600], ["h", 700]]);
const rankToY = new IMap<string, number>([["1", 700], ["2", 600], ["3", 500], ["4", 400], ["5", 300], ["6", 200], ["7", 100], ["8", 0]]);

const colToFileFlipped = new IMap<number, string>([[0, "h"], [1, "g"], [2, "f"], [3, "e"], [4, "d"], [5, "c"], [6, "b"], [7, "a"]]);
const rowToRankFlipped = new IMap<number, string>([[0, "1"], [1, "2"], [2, "3"], [3, "4"], [4, "5"], [5, "6"], [6, "7"], [7, "8"]]);
const xToFileFlipped = new IMap<number, string>([[0, "h"], [100, "g"], [200, "f"], [300, "e"], [400, "d"], [500, "c"], [600, "b"], [700, "a"]]);
const yToRankFlipped = new IMap<number, string>([[700, "8"], [600, "7"], [500, "6"], [400, "5"], [300, "4"], [200, "3"], [100, "2"], [0, "1"]]);
const fileToXFlipped = new IMap<string, number>([["h", 0], ["g", 100], ["f", 200], ["e", 300], ["d", 400], ["c", 500], ["b", 600], ["a", 700]]);
const rankToYFlipped = new IMap<string, number>([["8", 700], ["7", 600], ["6", 500], ["5", 400], ["4", 300], ["3", 200], ["2", 100], ["1", 0]]);

const theme = "Neo";
const themeToFiletype = new IMap<string, string>([["Neo", "png"]]);
const pieceToPieceFilename = new IMap<string, string>([
  ["r", "bR"],
  ["n", "bN"],
  ["b", "bB"],
  ["q", "bQ"],
  ["k", "bK"],
  ["p", "bP"],
  ["R", "wR"],
  ["N", "wN"],
  ["B", "wB"],
  ["Q", "wQ"],
  ["K", "wK"],
  ["P", "wP"],
]);

function rowColToSquare([row, col]: number[], flipped: boolean): string {
  if (flipped) {
    if (!colToFileFlipped.has(col) || !rowToRankFlipped.has(row)) {
      throw new Error('Invalid col or row');
    }
    return colToFileFlipped.get(col)! + rowToRankFlipped.get(row)!;
  } else {
    if (!colToFile.has(col) || !rowToRank.has(row)) {
      throw new Error('Invalid col or row');
    }
    return colToFile.get(col)! + rowToRank.get(row)!;
  }
}

function coordsToSquare([x, y]: number[], flipped: boolean): string {
  if (flipped) {
    if (!xToFileFlipped.has(x) || !yToRankFlipped.has(y)) {
      throw new Error('Invalid x or y coord');
    }
    return xToFileFlipped.get(x)! + yToRankFlipped.get(y)!;
  } else {
    if (!xToFile.has(x) || !yToRank.has(y)) {
      throw new Error('Invalid x or y coord');
    }
    return xToFile.get(x)! + yToRank.get(y)!;
  }
}

function squareToCoords(square: string, flipped: boolean): [number, number] {
  if (flipped) {
    return [fileToXFlipped.get(square[0])!, rankToYFlipped.get(square[1])!];
  } else {
    return [fileToX.get(square[0])!, rankToY.get(square[1])!];
  }
}

function getSvgCoords(e: React.MouseEvent<HTMLElement>) {
  const svgElement = document.getElementById("chess-board-svg");
  if (!svgElement || !(svgElement instanceof SVGGraphicsElement)) {
    throw new Error(`expected #chess-board-svg to be a SVGGraphicsElement, was ${svgElement && svgElement.constructor && svgElement.constructor.name || svgElement}`)
  }
  const ctm = (svgElement as SVGGraphicsElement).getScreenCTM()!;
  const svgX = (e.clientX - ctm.e) / ctm.a;
  const svgY = (e.clientY - ctm.f) / ctm.a;
  return [svgX, svgY];
}

function normaliseCoords([x, y]: number[]): number[] {
  return [(100 * Math.floor(x / 100)), (100 * Math.floor(y / 100))]
}

function getSquare(e: React.MouseEvent<HTMLElement>, flipped: boolean) {
  const svgCoords = getSvgCoords(e);
  const coords = normaliseCoords(svgCoords);
  const square = coordsToSquare(coords, flipped);
  return square;
}

function getPieceHref(piece: string): string {
  const pieceFilename = pieceToPieceFilename.get(piece);
  const themeFiletype = themeToFiletype.get(theme);
  return `img/pieces/${theme}/${pieceFilename}.${themeFiletype}`;
}

function getKnightArrowPointsAndRotateAngle(sourceSquare: string, destSquare: string, flipped: boolean): [number[][], number] {
  let [x, y] = squareToCoords(sourceSquare, flipped);
  let [destX, destY] = squareToCoords(destSquare, flipped);
  x += 50;
  y += 50;
  destX += 50;
  destY += 50;

  const halfStemWidth = 13.75;
  const arrowSideWidth = 18.75;
  const arrowHeadLength = 45;

  let rotateAngle = 0;
  let isLeftArrow = false;

  if (x == destX - 100 && y == destY - 200) {
    rotateAngle = 0;
    isLeftArrow = false;
  } else if (x == destX - 200 && y == destY - 100) {
    rotateAngle = 270;
    isLeftArrow = true;
  } else if (x == destX + 100 && y == destY - 200) {
    rotateAngle = 0;
    isLeftArrow = true;
  } else if (x == destX + 200 && y == destY - 100) {
    rotateAngle = 90;
    isLeftArrow = false;
  } else if (x == destX - 100 && y == destY + 200) {
    rotateAngle = 180;
    isLeftArrow = true;
  } else if (x == destX - 200 && y == destY + 100) {
    rotateAngle = 270;
    isLeftArrow = false;
  } else if (x == destX + 200 && y == destY + 100) {
    rotateAngle = 90;
    isLeftArrow = true;
  } else if (x == destX + 100 && y == destY + 200) {
    rotateAngle = 180;
    isLeftArrow = false;
  }

  let points = [];
  if (isLeftArrow) {
    points = [[x + halfStemWidth, y],
    [x + halfStemWidth, y + halfStemWidth + 200],
    [(x - 100) + arrowHeadLength, y + halfStemWidth + 200],
    [(x - 100) + arrowHeadLength, y + halfStemWidth + arrowSideWidth + 200],
    [x - 100, y + 200],
    [(x - 100) + arrowHeadLength, (y + 200) - halfStemWidth - arrowSideWidth],
    [(x - 100) + arrowHeadLength, (y + 200) - halfStemWidth],
    [x - halfStemWidth, (y + 200) - halfStemWidth],
    [x - halfStemWidth, y]];
  } else {
    points = [[x - halfStemWidth, y],
    [x - halfStemWidth, y + halfStemWidth + 200],
    [(x + 100) - arrowHeadLength, y + halfStemWidth + 200],
    [(x + 100) - arrowHeadLength, y + halfStemWidth + arrowSideWidth + 200],
    [x + 100, y + 200],
    [(x + 100) - arrowHeadLength, (y + 200) - halfStemWidth - arrowSideWidth],
    [(x + 100) - arrowHeadLength, (y + 200) - halfStemWidth],
    [x + halfStemWidth, (y + 200) - halfStemWidth],
    [x + halfStemWidth, y]];
  }

  return [points, rotateAngle];
}

function getStraightArrowPointsAndRotateAngle(sourceSquare: string, destSquare: string, flipped: boolean): [number[][], number] {
  let [x, y] = squareToCoords(sourceSquare, flipped);
  let [destX, destY] = squareToCoords(destSquare, flipped);
  x += 50;
  y += 50;
  destX += 50;
  destY += 50;

  const halfStemWidth = 13.75;
  const squareDiff = (Math.sqrt(Math.pow(destX - x, 2) + Math.pow(destY - y, 2))) / 100;
  const stemLength = 55 + (100 * (squareDiff - 1));
  const arrowSideWidth = 18.75;
  const arrowHeadLength = 45;
  const points = [[x - halfStemWidth, y],
  [x - halfStemWidth, y + stemLength],
  [x - halfStemWidth - arrowSideWidth, y + stemLength],
  [x, y + stemLength + arrowHeadLength],
  [x + halfStemWidth + arrowSideWidth, y + stemLength],
  [x + halfStemWidth, y + stemLength],
  [x + halfStemWidth, y]];
  const rotateAngle = -1 * (((Math.atan2(x - destX, y - destY) * 180) / Math.PI) - 180);
  return [points, rotateAngle];

}


function isKnightMove(sourceSquare: string, destSquare: string, flipped: boolean): boolean {
  const [x, y] = squareToCoords(sourceSquare, flipped);
  const [destX, destY] = squareToCoords(destSquare, flipped);
  return (x == destX - 100 && y == destY - 200) ||
    (x == destX - 200 && y == destY - 100) ||
    (x == destX + 100 && y == destY - 200) ||
    (x == destX + 200 && y == destY - 100) ||
    (x == destX - 100 && y == destY + 200) ||
    (x == destX - 200 && y == destY + 100) ||
    (x == destX + 200 && y == destY + 100) ||
    (x == destX + 100 && y == destY + 200);
}

function drawBoardSquares(boardState: IMap<string>, flipped: boolean) {
  let squareClasses = boardState.get("squareClasses");
  if (boardState.hasIn(["dragState", "piece"])) {
    squareClasses = squareClasses.set(boardState.getIn(["dragState", "piece", 1]), "original-square");
  }

  if (boardState.has("lastMove")) {
    const lastMove = boardState.get("lastMove");
    squareClasses = squareClasses
      .set(lastMove[0], "original-square")
      .set(lastMove[1], "original-square");
  }

  if (boardState.has("selectedPiece")) {
    squareClasses = squareClasses.set(boardState.getIn(["selectedPiece", 1]), "original-square");
  }

  return Array.from({ length: 64 }, (_, i) => {
    const row = Math.floor(i / 8)
    const column = i % 8
    const x = column * 100;
    const y = row * 100;
    const square = rowColToSquare([row, column], flipped);

    let c = "";
    if (squareClasses.has(square)) {
      c = squareClasses.get(square)!;
    } else if ((column % 2 == 0 && row % 2 == 0) || (column % 2 != 0 && row % 2 != 0)) {
      c = "square-light";
    } else {
      c = "square-dark";
    }

    return (<rect id={square} key={square} className={c} width="100" height="100" x={x} y={y} />);
  });
}

function drawFileCoordinates(flipped: boolean) {
  return Array.from({ length: 8 }, (_, i) => {
    const j = i + 1;
    const offset = i * 100
    const c = j % 2 == 0 ? "coordinate-dark" : "coordinate-light";
    const file = flipped ? colToFile.get(7 - i) : colToFile.get(i);
    return (<text className={c} key={file + offset} fontSize="20" x={offset + 80} y="795" >{file}</text>);
  });
}

function drawRankCoordinates(flipped: boolean) {
  return Array.from({ length: 8 }, (_, i) => {
    const j = i + 1;
    const offset = 800 - (j * 100)
    const c = j % 2 == 0 ? "coordinate-dark" : "coordinate-light";
    const rank = flipped ? 9 - j : j;
    return (<text className={c} key={j + offset} fontSize="20" x="5" y={offset + 20}>{rank}</text>);
  });
}


function drawArrow(sourceSquare: string, destSquare: string, flipped: boolean) {
  const id = sourceSquare + destSquare;

  let [x, y] = squareToCoords(sourceSquare, flipped);
  x += 50;
  y += 50;

  let points = [];
  let rotateAngle = 0;
  if (isKnightMove(sourceSquare, destSquare, flipped)) {
    [points, rotateAngle] = getKnightArrowPointsAndRotateAngle(sourceSquare, destSquare, flipped);
  } else {
    [points, rotateAngle] = getStraightArrowPointsAndRotateAngle(sourceSquare, destSquare, flipped);
  }

  const pointsStr = points.map(x => x.join(" ")).join(",");
  const transform = `rotate(${rotateAngle} ${x} ${y})`;
  return (<polygon id={id} key={id} className="arrow" points={pointsStr} transform={transform} />);
}

function drawArrows(arrows: ISet<IList<string>>, flipped: boolean) {
  return arrows.map(([sourceSquare, destSquare]: IList<string>) => drawArrow(sourceSquare, destSquare, flipped));
}

function drawPiece(piece: string, square: string, [x, y]: [number, number]) {
  const id = `${piece}-${square}`;
  const pieceHref = getPieceHref(piece);
  return (<image id={id} key={id} x={x} y={y} width="100" height="100" draggable="true" href={pieceHref} />);
}

function drawPieces(pieces: ISet<IList<string>>, dragState: IMap<string>, flipped: boolean) {
  return pieces.map(([piece, square]: IList<string>) => {
    if (square == dragState.getIn(["piece", 1])) {
      return drawPiece(piece, square, dragState.get("coords"));
    } else {
      return drawPiece(piece, square, squareToCoords(square, flipped));
    }
  });
}

function drawMoveHints(moveHints: string[], flipped: boolean) {
  if (moveHints) {
    return moveHints.map((hintSquare: string) => {
      let [x, y] = squareToCoords(hintSquare, flipped);
      x += 50;
      y += 50
      return (<circle key={hintSquare + "MoveHint"} cx={x} cy={y} r="15" className="move-hint" />);
    })
  }
}

function makeMove(boardState: IMap<string>, gameState: any, oldSquare: string, newSquare: string, clickMove: boolean, openModel: Function): IMap<string> {
  if ((gameState["validMoves"].has(oldSquare)) && gameState["validMoves"].get(oldSquare).includes(newSquare)) {
    let piece = boardState.getIn(["selectedPiece", 0]);
    let promotion = (piece === "P" && oldSquare[1] == "7" && newSquare[1] == "8") ||
      (piece === "p" && oldSquare[1] == "2" && newSquare[1] == "1");
    if (promotion) {
      openModel({ "isWhite": piece === "P", "move": `${oldSquare}${newSquare}` });
      return [boardState
        .set("dragState", new IMap<string>())
        .delete("selectedPiece")
        .delete("moveHints"), undefined];
    } else {
      return [boardState
        .set("dragState", new IMap<string>())
        .delete("selectedPiece")
        .delete("moveHints"), `${oldSquare}${newSquare}`];
    }
  } else {
    let newBoardState = boardState
      .set("dragState", new IMap<string>());
    if (clickMove) {
      newBoardState = newBoardState.delete("selectedPiece").delete("moveHints");
    }

    newBoardState = newBoardState.set("moveHints", gameState["validMoves"].get(newBoardState.getIn(["selectedPiece", 1])));

    return [newBoardState, undefined];
  }
}

function handleDrag(e: React.MouseEvent<HTMLElement>, boardState: IMap<string>, gameState: any, flipped: boolean): IMap<string> {
  const target = e.target;
  const pieceX = parseFloat(target.getAttributeNS("", "x"));
  const pieceY = parseFloat(target.getAttributeNS("", "y"));
  const [x, y] = getSvgCoords(e);
  const square = getSquare(e, flipped);
  const offsetX = x - pieceX;
  const offsetY = y - pieceY;

  const dragPiece = gameState["pieces"].filter((x: IList<string>) => x[1] == square)[0];

  return boardState
    .setIn(["dragState", "piece"], dragPiece)
    .setIn(["dragState", "coords"], [pieceX, pieceY])
    .setIn(["dragState", "offset"], [offsetX, offsetY])
    .set("selectedPiece", dragPiece)
    .set("moveHints", gameState["validMoves"].get(square));
}

function handleMakeMove(e: React.MouseEvent<HTMLElement>, boardState: IMap<string>, gameState: any, flipped: boolean, openModel: Function): IMap<string> {
  if (boardState.has("selectedPiece")) {
    const oldSquare = boardState.getIn(["selectedPiece", 1]);
    const newSquare = getSquare(e, flipped);
    return makeMove(boardState, gameState, oldSquare, newSquare, true, openModel);
  }

  return [boardState, undefined];
}

function handleHighlight(e: React.MouseEvent<HTMLElement>, square: string, boardState: IMap<string>): IMap<string> {
  if (boardState.get("squareClasses").has(square)) {
    return boardState.update("squareClasses", (x: IMap<string, string>) => x.delete(square));
  } else {
    let highlightClass = "";
    if (!(e.altKey || e.ctrlKey || e.shiftKey)) {
      highlightClass = "default-highlight";
    } else if (e.ctrlKey && !(e.altKey || e.shiftKey)) {
      highlightClass = "highlight-1";
    } else if (e.altKey && !(e.ctrlKey || e.shiftKey)) {
      highlightClass = "highlight-2";
    } else if (e.shiftKey && !(e.altKey || e.ctrlKey)) {
      highlightClass = "highlight-3";
    }

    return boardState.setIn(["squareClasses", square], highlightClass);
  }
}

function handleArrows(downSquare: string, upSquare: string, boardState: IMap<string>): IMap<string> {
  const arrow: IList<string> = new IList<string>([downSquare, upSquare]);
  if (boardState.get("arrows").has(arrow)) {
    return boardState.update("arrows", (x: ISet<IList<string>>) => x.delete(arrow));
  } else {
    return boardState.update("arrows", (x: ISet<IList<string>>) => x.add(arrow));
  }
}

function handleMouseDown(e: React.MouseEvent<HTMLElement>, clickState: Map<string, string>, gameState: any, boardState: IMap<string>, canMove: boolean, flipped: boolean, openModel: Function): IMap<String> {
  if (e.button == 0) { // Left Click
    let newBoardState = boardState
      .set("arrows", new ISet<IList<string>>())
      .set("squareClasses", new IMap<string, string>());

    if (canMove) {
      let newMove = undefined;

      if (e.target.attributes.draggable) {
        newBoardState = handleDrag(e, newBoardState, gameState, flipped);
      } else {
        [newBoardState, newMove] = handleMakeMove(e, newBoardState, gameState, flipped, openModel);
      }

      e.preventDefault();
      return [newBoardState, newMove];
    } else {
      return [newBoardState, undefined];
    }
  } else if (e.button == 2) { // Right Click
    clickState.set("right-mouse-down-square", getSquare(e, flipped));
  }

  return [boardState, undefined];
}

function handleMouseMove(e: React.MouseEvent<HTMLElement>, _: Map<string, string>, __: any, boardState: IMap<string>, canMove: boolean, flipped: boolean, openModel: Function): IMap<string> {
  if (boardState.hasIn(["dragState", "piece"]) && canMove) {
    const [coordX, coordY] = getSvgCoords(e);
    const [offsetX, offsetY] = boardState.getIn(["dragState", "offset"]);
    const newX = coordX - offsetX;
    const newY = coordY - offsetY;
    return [boardState.setIn(["dragState", "coords"], [newX, newY])];
  }

  return [boardState, undefined];
}

function handleMouseUp(e: React.MouseEvent<HTMLElement>, clickState: Map<string, string>, gameState: any, boardState: IMap<string>, canMove: boolean, flipped: boolean, openModel: Function): IMap<string> {
  let newBoardState = boardState;
  let newMove = undefined;

  if (e.button == 0 && newBoardState.hasIn(["dragState", "piece"]) && canMove) { // Left Click
    const dragPiece = newBoardState.getIn(["dragState", "piece"]);
    const oldSquare = dragPiece[1];
    const newSquare = getSquare(e, flipped);
    if (oldSquare == newSquare) {
      newBoardState = newBoardState
        .set("dragState", new IMap<string>())
        .set("moveHints", gameState["validMoves"].get(oldSquare));
    } else {
      [newBoardState, newMove] = makeMove(newBoardState, gameState, oldSquare, newSquare, false, openModel);
    }
  } else if (e.button == 2) { // Right Click
    e.preventDefault();

    if (clickState.has("right-mouse-down-square")) {
      const downSquare: string = clickState.get("right-mouse-down-square")!;
      const upSquare: string = getSquare(e, flipped);
      if (downSquare == upSquare) {
        newBoardState = handleHighlight(e, upSquare, newBoardState);
      } else {
        newBoardState = handleArrows(downSquare, upSquare, newBoardState);
      }
      clickState.delete("right-mouse-down-square");
    }
  }

  return [newBoardState, newMove];
}

function handleMouseEvent(
  e: React.MouseEvent<HTMLElement>,
  clickState: Map<string,
    string>,
  gameState: any,
  boardState: IMap<string>,
  setBoardState: Function,
  makeMove: Function,
  canMove: boolean,
  flipped: boolean,
  openModel: Function,
  f: Function,
) {
  const [newBoardState, newMove] = f(e, clickState, gameState, boardState, canMove, flipped, openModel);
  setBoardState(newBoardState);
  if (newMove) {
    makeMove(newMove);
  }
}

function makeDefaultBoardState() {
  return new IMap<string>({
    "squareClasses": new IMap<string, string>(),
    "arrows": new ISet<IList<string>>(),
    "dragState": new IMap<string>(),
  });
}

type BoardProps = {
  gameState: {
    gameId: string,
    pieces: IList<IList<string>>,
    validMoves: IMap<string, string[]>,
  },
  lastMove: string[],
  makeMove: Function,
  canMove: boolean,
  flipped: boolean,
}

// Make sure to bind modal to your appElement (https://reactcommunity.org/react-modal/accessibility/)
Modal.setAppElement('#root');

function drawModal(isOpen: boolean, modalState: any, onRequestClose: Function, makeMove: Function) {
  let pieces = [];
  if (modalState["isWhite"]) {
    pieces = ["Q", "N", "R", "B"];
  } else {
    pieces = ["q", "n", "r", "b"];
  }

  return (
    <Modal
      isOpen={isOpen}
      onRequestClose={onRequestClose}
      contentLabel="Example Modal"
      className="Modal"
      overlayClassName="Overlay"
    >
      {pieces.map((p) => <img
        id={`white-${p}-promotion`}
        key={`white-${p}-promotion`}
        width="100"
        height="100"
        onClick={() => {
          makeMove(`${modalState["move"]}${p.toLowerCase()}`);
          onRequestClose();
        }}
        src={getPieceHref(p)} />)}
    </Modal>
  );
}

function Board(props: BoardProps) {
  // FIXME: Will having all the state in one map have performance implications for rendering. i.e. will it still be able to do smart things like see that only arrows has changed.
  const [boardState, setBoardState] = useState(makeDefaultBoardState());
  const [modalIsOpen, setmodalIsOpen] = useState(false);
  const [modalState, setModalState] = useState({});
  let gameState = props.gameState;
  let makeMove = props.makeMove;
  let canMove = props.canMove;

  let clickState = new Map<string, string>();

  const openModel = function(modalState: any) {
    setmodalIsOpen(true);
    setModalState(modalState);
  };

  // FIXME: can only capture when dragging

  return (
    <div className="board">
      {drawModal(modalIsOpen, modalState, () => setmodalIsOpen(false), makeMove)}
      <svg
        id="chess-board-svg"
        width="800"
        height="800"
        viewBox="0 0 800 800"
        onMouseDown={(e: React.MouseEvent<HTMLElement>) => handleMouseEvent(e, clickState, gameState, boardState, setBoardState, makeMove, canMove, props.flipped, openModel, handleMouseDown)}
        onMouseMove={(e: React.MouseEvent<HTMLElement>) => handleMouseEvent(e, clickState, gameState, boardState, setBoardState, makeMove, canMove, props.flipped, openModel, handleMouseMove)}
        onMouseUp={(e: React.MouseEvent<HTMLElement>) => handleMouseEvent(e, clickState, gameState, boardState, setBoardState, makeMove, canMove, props.flipped, openModel, handleMouseUp)}
        onContextMenu={(e: React.MouseEvent<HTMLElement>) => { e.preventDefault(); }}
      >
        {drawBoardSquares(boardState.set("lastMove", props.lastMove), props.flipped)}
        {drawFileCoordinates(props.flipped)}
        {drawRankCoordinates(props.flipped)}
        {drawArrows(boardState.get("arrows"), props.flipped)}
        {drawMoveHints(boardState.get("moveHints"), props.flipped)}
        {drawPieces(gameState["pieces"], boardState.get("dragState"), props.flipped)}
      </svg>
    </div>
  );
}

export default Board;
