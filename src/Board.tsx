import { Map as IMap, Set as ISet, List as IList } from 'immutable';
import { useState, React } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./Board.css";

const colToFile = new IMap<number, string>([[0, "a"], [1, "b"], [2, "c"], [3, "d"], [4, "e"], [5, "f"], [6, "g"], [7, "h"]]);
const rowToRank = new IMap<number, string>([[0, "8"], [1, "7"], [2, "6"], [3, "5"], [4, "4"], [5, "3"], [6, "2"], [7, "1"]]);
const xToFile = new IMap<number, string>([[0, "a"], [100, "b"], [200, "c"], [300, "d"], [400, "e"], [500, "f"], [600, "g"], [700, "h"]]);
const yToRank = new IMap<number, string>([[700, "1"], [600, "2"], [500, "3"], [400, "4"], [300, "5"], [200, "6"], [100, "7"], [0, "8"]]);
const fileToX = new IMap<string, number>([["a", 0], ["b", 100], ["c", 200], ["d", 300], ["e", 400], ["f", 500], ["g", 600], ["h", 700]]);
const rankToY = new IMap<string, number>([["1", 700], ["2", 600], ["3", 500], ["4", 400], ["5", 300], ["6", 200], ["7", 100], ["8", 0]]);

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

function rowColToSquare([row, col]: number[]): string {
  if (!colToFile.has(col) || !rowToRank.has(row)) {
    throw new Error('Invalid col or row');
  }
  return colToFile.get(col)! + rowToRank.get(row)!;
}

function coordsToSquare([x, y]: number[]): string {
  if (!xToFile.has(x) || !yToRank.has(y)) {
    throw new Error('Invalid x or y coord');
  }
  return xToFile.get(x)! + yToRank.get(y)!;
}

function squareToCoords(square: string): [number, number] {
  return [fileToX.get(square[0])!, rankToY.get(square[1])!];
}

function getSvgCoords(e: React.MouseEvent<HTMLElement>) {
  const ctm = document.getElementById("chess-board-svg").getScreenCTM();
  const svgX = (e.clientX - ctm.e) / ctm.a;
  const svgY = (e.clientY - ctm.f) / ctm.a;
  return [svgX, svgY];
}

function normaliseCoords([x, y]: number[]): number[] {
  return [(100 * Math.floor(x / 100)), (100 * Math.floor(y / 100))]
}

function getSquare(e: React.MouseEvent<HTMLElement>) {
  const svgCoords = getSvgCoords(e);
  const coords = normaliseCoords(svgCoords);
  const square = coordsToSquare(coords);
  return square;
}

function getPieceHref(piece: string): string {
  const pieceFilename = pieceToPieceFilename.get(piece);
  const themeFiletype = themeToFiletype.get(theme);
  return `img/pieces/${theme}/${pieceFilename}.${themeFiletype}`;
}

function getKnightArrowPointsAndRotateAngle(sourceSquare: string, destSquare: string): [number[][], number] {
  let [x, y] = squareToCoords(sourceSquare);
  let [destX, destY] = squareToCoords(destSquare);
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

function getStraightArrowPointsAndRotateAngle(sourceSquare: string, destSquare: string): [number[][], number] {
  let [x, y] = squareToCoords(sourceSquare);
  let [destX, destY] = squareToCoords(destSquare);
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


function isKnightMove(sourceSquare: string, destSquare: string): boolean {
  const [x, y] = squareToCoords(sourceSquare);
  const [destX, destY] = squareToCoords(destSquare);
  return (x == destX - 100 && y == destY - 200) ||
    (x == destX - 200 && y == destY - 100) ||
    (x == destX + 100 && y == destY - 200) ||
    (x == destX + 200 && y == destY - 100) ||
    (x == destX - 100 && y == destY + 200) ||
    (x == destX - 200 && y == destY + 100) ||
    (x == destX + 200 && y == destY + 100) ||
    (x == destX + 100 && y == destY + 200);
}

function drawBoardSquares(boardState: IMap<string>) {
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
    const square = rowColToSquare([row, column]);

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

function drawFileCoordinates() {
  return Array.from({ length: 8 }, (_, i) => {
    const j = i + 1;
    const offset = i * 100
    const c = j % 2 == 0 ? "coordinate-dark" : "coordinate-light";
    const file = colToFile.get(i)
    return (<text className={c} key={file + offset} fontSize="20" x={offset + 80} y="795" >{file}</text>);
  });
}

function drawRankCoordinates() {
  return Array.from({ length: 8 }, (_, i) => {
    const j = i + 1;
    const offset = 800 - (j * 100)
    const c = j % 2 == 0 ? "coordinate-dark" : "coordinate-light";
    return (<text className={c} key={j + offset} fontSize="20" x="5" y={offset + 20}>{j}</text>);
  });
}


function drawArrow(sourceSquare: string, destSquare: string) {
  const id = sourceSquare + destSquare;

  let [x, y] = squareToCoords(sourceSquare);
  x += 50;
  y += 50;

  let points = [];
  let rotateAngle = 0;
  if (isKnightMove(sourceSquare, destSquare)) {
    [points, rotateAngle] = getKnightArrowPointsAndRotateAngle(sourceSquare, destSquare);
  } else {
    [points, rotateAngle] = getStraightArrowPointsAndRotateAngle(sourceSquare, destSquare);
  }

  const pointsStr = points.map(x => x.join(" ")).join(",");
  const transform = `rotate(${rotateAngle} ${x} ${y})`;
  return (<polygon id={id} key={id} className="arrow" points={pointsStr} transform={transform} />);
}

function drawArrows(arrows: ISet<IList<string>>) {
  return arrows.map(([sourceSquare, destSquare]) => drawArrow(sourceSquare, destSquare));
}

function drawPiece(piece: string, square: string, [x, y]: [number, number]) {
  const id = `${piece}-${square}`;
  const pieceHref = getPieceHref(piece);
  return (<image id={id} key={id} x={x} y={y} width="100" height="100" draggable="true" href={pieceHref} />);
}

function drawPieces(pieces: ISet<IList<string>>, dragState: IMap<string>) {
  return pieces.map(([piece, square]) => {
    if (square == dragState.getIn(["piece", 1])) {
      return drawPiece(piece, square, dragState.get("coords"));
    } else {
      return drawPiece(piece, square, squareToCoords(square));
    }
  });
}

function drawMoveHints(moveHints: string[]) {
  if (moveHints) {
    return moveHints.map((hintSquare: string) => {
      let [x, y] = squareToCoords(hintSquare);
      x += 50;
      y += 50
      return (<circle key={hintSquare + "MoveHint"} cx={x} cy={y} r="15" className="move-hint" />);
    })
  }
}

function makeMove(boardState: IMap<string>, setBoardState, oldSquare: string, newSquare: string) {
  if (boardState.getIn(["validMoves", oldSquare]) && boardState.getIn(["validMoves", oldSquare]).includes(newSquare)) {
    setBoardState(boardState
      .set("dragState", new IMap<string>())
      .update("pieces", (x: ISet<IList<string>>) => x.map((y: IList<string>) => y[1] == oldSquare ? [y[0], newSquare] : y))
      .delete("selectedPiece")
      .delete("moveHints")
      .set("lastMove", [oldSquare, newSquare])
      .set("validMoves", new IMap<string, string[]>()) // FIXME: Need to interface with the actual chess engine to get the new valid moves
    );
  } else {
    setBoardState(boardState
      .set("dragState", new IMap<string>())
      .delete("selectedPiece") // FIXME: (DEVON) we only want to do this if it was a click move not a drag move.
      .delete("moveHints")
    );
  }
}

function handleDrag(e: React.MouseEvent<HTMLElement>, boardState: IMap<string>, setBoardState) {
  const target = e.target;
  const pieceX = parseFloat(target.getAttributeNS("", "x"));
  const pieceY = parseFloat(target.getAttributeNS("", "y"));
  const [x, y] = getSvgCoords(e);
  const square = getSquare(e);
  const offsetX = x - pieceX;
  const offsetY = y - pieceY;

  const dragPiece = boardState.get("pieces").filter((x: IList<string>) => x[1] == square).first();

  setBoardState(boardState
    .setIn(["dragState", "piece"], dragPiece)
    .setIn(["dragState", "coords"], [pieceX, pieceY])
    .setIn(["dragState", "offset"], [offsetX, offsetY])
    .set("moveHints", boardState.getIn(["validMoves", square]))
  );
}

function handleMakeMove(e: React.MouseEvent<HTMLElement>, boardState: IMap<string>, setBoardState) {
  if (boardState.has("selectedPiece")) {
    const oldSquare = boardState.getIn(["selectedPiece", 1]);
    const newSquare = getSquare(e);
    makeMove(boardState, setBoardState, oldSquare, newSquare);
  }
}

function handleHighlight(e: React.MouseEvent<HTMLElement>, square: string, boardState: IMap<string>, setBoardState) {
  if (boardState.get("squareClasses").has(square)) {
    setBoardState(boardState.update("squareClasses", (x: IMap<string, string>) => x.delete(square)));
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

    setBoardState(boardState.setIn(["squareClasses", square], highlightClass))
  }
}

function handleArrows(downSquare: string, upSquare: string, boardState: IMap<string>, setBoardState) {
  const arrow: IList<string> = new IList<string>([downSquare, upSquare]);
  if (boardState.get("arrows").has(arrow)) {
    setBoardState(boardState.update("arrows", (x: ISet<IList<string>>) => x.delete(arrow)))
  } else {
    setBoardState(boardState.update("arrows", (x: ISet<IList<string>>) => x.add(arrow)))
  }
}

function handleMouseDown(e: React.MouseEvent<HTMLElement>, clickState: Map<string, string>, boardState: IMap<string>, setBoardState) {
  if (e.button == 0) { // Left Click
    setBoardState(boardState
      .set("arrows", new ISet<IList<string>>())
      .set("squareClasses", new IMap<string, string>()));

    if (e.target.attributes.draggable) {
      handleDrag(e, boardState, setBoardState);
    } else {
      handleMakeMove(e, boardState, setBoardState);
    }

    e.preventDefault();
  } else if (e.button == 2) { // Right Click
    clickState.set("right-mouse-down-square", getSquare(e));
  }
}

function handleMouseMove(e: React.MouseEvent<HTMLElement>, boardState: IMap<string>, setBoardState) {
  if (boardState.hasIn(["dragState", "piece"])) {
    const [coordX, coordY] = getSvgCoords(e);
    const [offsetX, offsetY] = boardState.getIn(["dragState", "offset"]);
    const newX = coordX - offsetX;
    const newY = coordY - offsetY;
    setBoardState(boardState.setIn(["dragState", "coords"], [newX, newY]));
  }
}

function handleMouseUp(e: React.MouseEvent<HTMLElement>, clickState: Map<string, string>, boardState: IMap<string>, setBoardState) {
  if (e.button == 0 && boardState.hasIn(["dragState", "piece"])) { // Left Click
    const dragPiece = boardState.getIn(["dragState", "piece"]);
    const oldSquare = dragPiece[1];
    const newSquare = getSquare(e);
    if (oldSquare == newSquare) {
      setBoardState(boardState
        .set("dragState", new IMap<string>())
        .set("selectedPiece", dragPiece)
        .set("moveHints", boardState.getIn(["validMoves", oldSquare]))
      );
    } else {
      makeMove(boardState, setBoardState, oldSquare, newSquare);
    }
  } else if (e.button == 2) { // Right Click
    e.preventDefault();
    const downSquare: string = clickState.get("right-mouse-down-square"); // FIXME: handle undefined.
    const upSquare: string = getSquare(e);
    if (downSquare == upSquare) {
      handleHighlight(e, upSquare, boardState, setBoardState);
    } else {
      handleArrows(downSquare, upSquare, boardState, setBoardState);
    }
    clickState.delete("right-mouse-down-square");
  }
}

function makeDefaultBoardState() {
  return new IMap<string>({
    "squareClasses": new IMap<string, string>(),
    "arrows": new ISet<IList<string>>(),
    "pieces": new ISet<IList<string>>([
      ["r", "a8"], ["n", "b8"], ["b", "c8"], ["q", "d8"], ["k", "e8"], ["b", "f8"], ["n", "g8"], ["r", "h8"],
      ["p", "a7"], ["p", "b7"], ["p", "c7"], ["p", "d7"], ["p", "e7"], ["p", "f7"], ["p", "g7"], ["p", "h7"],
      ["P", "a2"], ["P", "b2"], ["P", "c2"], ["P", "d2"], ["P", "e2"], ["P", "f2"], ["P", "g2"], ["P", "h2"],
      ["R", "a1"], ["N", "b1"], ["B", "c1"], ["Q", "d1"], ["K", "e1"], ["B", "f1"], ["N", "g1"], ["R", "h1"]
    ]),
    "validMoves": new IMap<string, string[]>(
      {
        "a2": ["a3", "a4"],
        "b2": ["b3", "b4"],
        "c2": ["c3", "c4"],
        "d2": ["d3", "d4"],
        "e2": ["e3", "e4"],
        "f2": ["f3", "f4"],
        "g2": ["g3", "g4"],
        "h2": ["h3", "h4"],
        "b1": ["a3", "c3"],
        "g1": ["f3", "h3"],
      }
    ),
    "dragState": new IMap<string>(),
  });
}

function Board() {
  // FIXME: Will having all the state in one map have performance implications for rendering. i.e. will it still be able to do smart things like see that only arrows has changed.
  const [boardState, setBoardState] = useState(makeDefaultBoardState());

  let clickState = new Map<string, string>();

  return (
    <div className="container">
      <h1>Welcome to Rust Chess!</h1>

      <svg
        id="chess-board-svg"
        width="800"
        height="800"
        viewBox="0 0 800 800"
        onMouseDown={(e: React.MouseEvent<HTMLElement>) => handleMouseDown(e, clickState, boardState, setBoardState)}
        onMouseMove={(e: React.MouseEvent<HTMLElement>) => handleMouseMove(e, boardState, setBoardState)}
        onMouseUp={(e: React.MouseEvent<HTMLElement>) => handleMouseUp(e, clickState, boardState, setBoardState)}
        onContextMenu={(e: React.MouseEvent<HTMLElement>) => { e.preventDefault(); }}
      >
        {drawBoardSquares(boardState)}
        {drawFileCoordinates()}
        {drawRankCoordinates()}
        {drawArrows(boardState.get("arrows"))}
        {drawPieces(boardState.get("pieces"), boardState.get("dragState"))}
        {drawMoveHints(boardState.get("moveHints"))}
      </svg>
    </div>
  );
}

export default Board;
