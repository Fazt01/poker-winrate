import styles from './WinrateChart.module.css'
import type {Solution} from "../types/Solution.ts";
import type {Card} from "../types/Card.tsx";
import Spinner from "./Spinner.tsx";
import {cardToText} from "../types/Card.ts";

export default function WinrateChart(
  {
    solution,
    hand,
    isLoading,
    errorText,
  }: {
    solution: Solution | null,
    hand: (Card | null)[],
    isLoading: boolean,
    errorText: string | null
  }
) {
  let segments = []
  let myHandSegment = null
  if (solution !== null) {
    const segmentWidth = 1 / solution.hands.length
    for (const [i, otherHand] of solution.hands.entries()) {
      const x = i / solution.hands.length
      const title = cardToText(otherHand.hand[0]) + cardToText(otherHand.hand[1])
      segments.push(
        <rect
          key={`${i}-loss`}
          className={`${styles.segment} ${styles.loss}`}
          x={x}
          y="0"
          width={segmentWidth}
          height={otherHand.beatsMeCount / solution.boardPossibilities}
        >
          <title>{title}</title>
        </rect>,
        <rect
          key={`${i}-tie`}
          className={`${styles.segment} ${styles.tie}`}
          x={x}
          y={otherHand.beatsMeCount / solution.boardPossibilities}
          width={segmentWidth}
          height={1 - (otherHand.beatsMeCount + otherHand.isBeatenCount) / solution.boardPossibilities}
        >
          <title>{title}</title>
        </rect>,
        <rect
          key={`${i}-win`}
          className={`${styles.segment} ${styles.win}`}
          x={x}
          y={1 - otherHand.isBeatenCount / solution.boardPossibilities}
          width={segmentWidth}
          height={otherHand.isBeatenCount / solution.boardPossibilities}
        >
          <title>{title}</title>
        </rect>,
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
  } else {
    const fakeHandsCount = 1000;
    const segmentWidth = 1 / fakeHandsCount
    for (let i = 0; i < fakeHandsCount; i++) {
      const x = i / fakeHandsCount
      segments.push(
        <rect
          key={`${i}-loss`}
          className={`${styles.segment} ${styles.loss}`}
          x={x}
          y="0"
          width={segmentWidth}
          height="0"
        />,
        <rect
          key={`${i}-tie`}
          className={`${styles.segment} ${styles.tie}`}
          x={x}
          y="0"
          width={segmentWidth}
          height="0"
        />,
        <rect
          key={`${i}-win`}
          className={`${styles.segment} ${styles.win}`}
          x={x}
          y="0"
          width={segmentWidth}
          height="0"
        />,
      )
    }
    myHandSegment = (<rect
      key="myhand"
      className={`${styles["my-hand"]} ${styles.segment}`}
      x="-0.1"
      y="-0.1"
      width={segmentWidth}
      height="1.2"
    />)
  }

    return <div className={styles.chart}>
      {
        isLoading && (<div className={styles.spinner}>
          <Spinner/>
        </div>)
      }
      {
        errorText && <div className={styles.error}>
          {errorText}
        </div>
      }

      <svg
        preserveAspectRatio="none"
        className={styles.chart}
        width="100%"
        height="300"
        viewBox={`0 -0.1 1 1.2`}
        shapeRendering="crispEdges"
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