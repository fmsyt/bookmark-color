type RGB = [number, number, number];

export type ColorMatrixProps = {
  colors: RGB[];
  gap?: number | string;
  width?: number | string;
  height?: number | string;
  minWidth?: number | string;
  minHeight?: number | string;
};

export default function ColorMatrix(props: ColorMatrixProps) {

  const sideLength = Math.ceil(Math.sqrt(props.colors.length));

  return (
    <div
      style={{
        display: "grid",
        gridTemplateColumns: `repeat(${sideLength}, 1fr)`,
        gridTemplateRows: `repeat(${sideLength}, 1fr)`,
        gap: props.gap || "2px",
        width: props.width || "100%",
        height: props.height || "100%",
        minWidth: props.minWidth || "20px",
        minHeight: props.minHeight || "20px",
      }}
    >
      {props.colors.map(([r, g, b], index) => (
        <div
          key={index}
          className="color-box"
          style={{
            backgroundColor: `rgb(${r}, ${g}, ${b})`,
          }}
        ></div>
      ))}
    </div>
  );
}
