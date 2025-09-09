import './App.css'
import Selection from "./components/Selection.tsx";
import Hand from "./components/Hand.tsx";
import Board from "./components/Board.tsx";
import type {Card} from "./types/Card.ts";
import {useSelectedCards} from "./states/SelectedCards.ts";
import WinrateChart from "./components/WinrateChart.tsx";
import {mock_solution} from "./mock_data/solution.ts";
import solve from "./wasm/Solution.ts"

function App() {
  const hand = useSelectedCards(2, 0)
  const board = useSelectedCards(5)

  let solution = mock_solution
  try {
    solution = solve({
      hand: hand.state.cards,
      board: board.state.cards,
    })
    console.log("success", solution)
  } catch (e) {
    console.log("error", e)
  }

  return (
    <>
      <Selection onCardSelected={(card: Card) => {
        if (hand.setSelectedCard(card)) {
          board.setSelectedSlot(0)
        }
        board.setSelectedCard(card)
      }}/>
      <h2>Hand</h2>
      <Hand state={hand.state} setSelectedSlot={
        (i) => {
          hand.setSelectedSlot(i)
          board.setSelectedSlot(null)
        }
      } clearCardAt={hand.clearCardAt} />
      <h2>Board</h2>
      <Board state={board.state} setSelectedSlot={
        (i) => {
          board.setSelectedSlot(i)
          hand.setSelectedSlot(null)
        }
      } clearCardAt={board.clearCardAt}/>
      <WinrateChart solution={solution} hand={hand.state.cards}/>
    </>
  )
}

export default App
