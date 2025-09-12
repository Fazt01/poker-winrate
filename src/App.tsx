import './App.css'
import Selection from "./components/Selection.tsx";
import Hand from "./components/Hand.tsx";
import Board from "./components/Board.tsx";
import type {Card} from "./types/Card.ts";
import {useSelectedCards} from "./states/SelectedCards.ts";
import WinrateChart from "./components/WinrateChart.tsx";
import solve from "./wasm/Solution.ts"
import {useEffect, useState} from "react";
import {AbortSignal} from "../rust-wasm/pkg";
import type {Solution} from "./types/Solution.ts";

function App() {
  const hand = useSelectedCards(2, 0)
  const board = useSelectedCards(5)
  const [solution, setSolution] = useState<Solution | null>(null)
  const [isLoading, setIsLoading] = useState(false)
  const [errorText, setErrorText] = useState<string | null>(null)

  useEffect(() => {
    const cancellationToken = new AbortSignal()
    setIsLoading(true)
    setErrorText(null)
    const f = async () => {
      try {
        const solution = await solve(cancellationToken, {
          hand: hand.state.cards,
          board: board.state.cards,
        })
        if (cancellationToken.aborted) {
          console.log("solve: discarding stale solution", solution)
          return
        }
        setSolution(solution)
        setErrorText(null)
        setIsLoading(false)
        console.log("solve: success", solution)
      } catch (e) {
        if (cancellationToken.aborted) {
          console.log("solve: discarding stale error: ", e)
          return
        }
        console.log("solve: error:", e)
        setSolution(null)
        if (typeof e == "string") {
          setErrorText(e)
        } else {
          setErrorText(null)
        }
        setIsLoading(false)
      }
    }
    f();
    return () => {
      cancellationToken.abort()
    }
  }, [
    hand.state.cards,
    board.state.cards,
  ])

  return (
    <>
      <Selection onCardSelected={(card: Card) => {
        if (hand.setSelectedCard(card)) {
          board.setSelectedSlot(0)
        }
        board.setSelectedCard(card)
      }}/>
      <h2>Hand</h2>
      <Hand state={hand.state} setSelectedSlot={i => {
        hand.setSelectedSlot(i)
        board.setSelectedSlot(null)
      }} clearCardAt={i => {
        hand.clearCardAt(i)
        hand.setSelectedSlot(i)
        board.setSelectedSlot(null)
      }}/>
      <h2>Board</h2>
      <Board state={board.state} setSelectedSlot={i => {
          board.setSelectedSlot(i)
          hand.setSelectedSlot(null)
      }} clearCardAt={i => {
        board.clearCardAt(i)
        board.setSelectedSlot(i)
        hand.setSelectedSlot(null)
      }}/>
      <WinrateChart
        solution={solution}
        isLoading={isLoading}
        errorText={errorText}
      />
    </>
  )
}

export default App
