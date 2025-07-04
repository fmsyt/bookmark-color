import "./pallet.css";

export type Props = {
  points?: number[];
  width?: string | number;
  height?: string | number;
  size?: number | string;
}

export default function ColorCircle(props: Props) {
  // NOTE: HSVはマトリックスのX座標、Y座標に対応している
  return (
    <div
      style={{
        position: "relative",
        width: props.size || props.width || '100%',
        height: props.size || props.height || '100%',
      }}
    >
      {/* Circle */}
      <div
        className="hsl-pallet-circle"
        style={{
          width: '100%',
          height: '100%',
          position: 'absolute',
        }}
      >
      </div>

      {/* Square */}
      <div
        style={{
          width: "55%",
          height: "55%",
          position: 'absolute',
          top: "50%",
          left: "50%",
          transform: "translateY(-50%) translateX(-50%)",
        }}
      >
        <div
          className="hsl-pallet-square"
        >
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
    </div>
  )

}
