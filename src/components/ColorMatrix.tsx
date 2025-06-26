type RGB = [number, number, number];

export type ColorMatrixProps = {
  colors: RGB[];
  gap?: number | string;
  cellSize?: number | string;
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
      }}
    >
      {props.colors.map(([r, g, b], index) => (
        <div
          key={index}
          className="color-box"
          style={{
            backgroundColor: `rgb(${r}, ${g}, ${b})`,
            width: props.cellSize || "1em",
            height: props.cellSize || "1em"
          }}
        ></div>
      ))}
    </div>
  );
}
