import './SidePanel.css'

type SidePanelProps = {
  moves: string[];
}

function SidePanel(props: SidePanelProps) {
  var movesArray: string[] = [];
  for (var i = 0; i < props.moves.length; i += 2) {
    movesArray.push(`${i / 2 + 1}.`);
    movesArray.push(props.moves[i]);

    if (i + 1 < props.moves.length) {
      movesArray.push(props.moves[i + 1]);
    }

    // FIXME: add some padding. the p element seems to swallow it tho.
  }
  let moves = movesArray.join(" ");
  return (
    <div className="side-panel">
      <div className="moves">
        <p>{moves}</p>
      </div>
      <div className="controls">
        <button>start</button>
        <button>previous</button>
        <button>next</button>
        <button>end</button>
        <button>flip</button>
      </div>
    </div>
  );
}

export default SidePanel;
