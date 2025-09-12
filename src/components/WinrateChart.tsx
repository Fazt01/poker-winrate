import styles from './WinrateChart.module.css'
import type {Solution} from "../types/Solution.ts";
import Spinner from "./Spinner.tsx";
import {cardToText} from "../types/Card.ts";

export default function WinrateChart(
  {
    solution,
    isLoading,
    errorText,
  }: {
    solution: Solution | null,
    isLoading: boolean,
    errorText: string | null
  }
) {
  let segments = []
  let myHandSegment = null
  if (solution !== null) {
    const winOrTieCount = solution.hands.length - solution.loseCount
    const accurateMyHandWidth = (winOrTieCount - solution.winCount) / solution.hands.length
    const minimumMyHandWidth = 1/100
    const myHandWidth = Math.max(minimumMyHandWidth, accurateMyHandWidth)
    const centerX = ((solution.winCount + winOrTieCount) / 2 ) / solution.hands.length
    segments.push(<rect
      key="myhand"
      className={`${styles["my-hand"]} ${styles.segment}`}
      x={centerX - myHandWidth/2}
      y="-0.1"
      width={myHandWidth}
      height="1.2"
    />)

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
    }
  } else {
    const fakeHandsCount = 1000;
    const segmentWidth = 1 / fakeHandsCount

    segments.push(<rect
      key="myhand"
      className={`${styles["my-hand"]} ${styles.segment}`}
      x="-0.1"
      y="-0.1"
      width={segmentWidth}
      height="1.2"
    />)
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
