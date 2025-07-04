import ColorSquare from "./ColorSquare";
import "./pallet.css";

type RGB = [number, number, number];

export type Props = {
  points?: RGB[];
  width?: string | number;
  height?: string | number;
  size?: number | string;
}

export default function ColorCircle(props: Props) {
  return (
    <div
      style={{
        position: "relative",
        width: props.size || props.width || '100%',
        height: props.size || props.height || '100%',
      }}
    >
      {/* Circle */}
      <div className="hsl-pallet-circle w-full h-full absolute" />

      {/* Square */}
      <ColorSquare
        size="55%"
        points={props.points}
      />
    </div>
  )

}
