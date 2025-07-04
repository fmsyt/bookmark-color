import "./pallet.css";

type RGB = [number, number, number];

type Props = {
  size?: string | number;
  width?: string | number;
  height?: string | number;
  points?: RGB[];
}

export default function ColorSquare(props: Props) {
  // NOTE: HSVはマトリックスのX座標、Y座標に対応している
  return (
    <div
      style={{
        width: props.size || props.width || '100%',
        height: props.size || props.height || '100%',
        position: "relative",
        top: "50%",
        left: "50%",
        transform: "translateY(-50%) translateX(-50%)",
      }}
    >
      <div className="hsl-pallet-square">
        <div
          className="hsl-pallet-square hue"
          style={{
            backgroundColor: "#F00"
          }}
        />
        <div className="hsl-pallet-square saturation" />
        <div className="hsl-pallet-square lightness" />
      </div>
    </div>
  );
}
