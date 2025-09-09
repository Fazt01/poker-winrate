import styles from './WinrateChart.module.css'
import type {Solution} from "../types/Solution.ts";
import type {Card} from "../types/Card.tsx";

export default function WinrateChart(
  {
    solution,
    hand,
  }: {
    solution: Solution
    hand: (Card | null)[]
  }
) {
  let segments = []
  const segmentWidth = 1 / solution.hands.length
  let myHandSegment = null
  for (const [i, otherHand] of solution.hands.entries()) {
    const x = i / solution.hands.length
    segments.push(
      <rect
        key={`${i}-loss`}
        className={`${styles.segment} ${styles.loss}`}
        x={x}
        y="0"
        width={segmentWidth}
        height={otherHand.beatsMeCount / solution.boardPossibilities}
      />,
      <rect
        key={`${i}-tie`}
        className={`${styles.segment} ${styles.tie}`}
        x={x}
        y={otherHand.beatsMeCount / solution.boardPossibilities}
        width={segmentWidth}
        height={1 - (otherHand.beatsMeCount + otherHand.isBeatenCount) / solution.boardPossibilities}
      />,
      <rect
        key={`${i}-win`}
        className={`${styles.segment} ${styles.win}`}
        x={x}
        y={1 - otherHand.isBeatenCount / solution.boardPossibilities}
        width={segmentWidth}
        height={otherHand.isBeatenCount / solution.boardPossibilities}
      />,
    )
    if (isSameHand(hand, otherHand.hand)) {
      myHandSegment = (<rect
        key="myhand"
        className={`${styles["my-hand"]} ${styles.segment}`}
        x={x}
        y="-0.1"
        width={segmentWidth}
        height="1.2"
      />)
    }
  }

  return <div className={styles.chart}>
    <svg
      preserveAspectRatio="none"
      className={styles.chart}
      width="100%"
      height="300"
      viewBox={`0 -0.1 1 1.2`}
    >
      {segments}
      {myHandSegment}
    </svg>
  </div>
}

function isSameHand(l: (Card | null)[], r: (Card | null)[]): boolean {
  return (
    l[0]?.rank === r[0]?.rank
    && l[0]?.suit === r[0]?.suit
    && l[1]?.rank === r[1]?.rank
    && l[1]?.suit === r[1]?.suit
  )
}