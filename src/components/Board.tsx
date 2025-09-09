import type {SelectedCardsProps} from "../states/SelectedCards.ts";
import SelectedCards from "./SelectedCards.tsx";

export default function Board(props: SelectedCardsProps) {
  return <SelectedCards count={5} props={props}/>
}