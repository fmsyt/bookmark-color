import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import "./App.css";
import reactLogo from "./assets/react.svg";
import ColorMatrix from "./components/ColorMatrix";
import { ColorPickerProvider, useColorPicker } from "./lib/picker";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  const [pickMargin, setPickMargin] = useState(2);

  const [point, setPoint] = useState({
    x: 0,
    y: 0,
  });

  const [colors, setColors] = useState<[number, number, number][]>([]);
  const [getPointError, setGetPointError] = useState("");

  async function greet() {
    setGreetMsg(await invoke("greet", { name }));
  }

  async function getPoint() {
    try {
      setPoint(await invoke("get_point"));
    } catch (error) {
      console.error("Error invoking get_point:", error);
      setGetPointError("Failed to get point. Check the console for details.");
    }
  }

  // 1秒ごとにpick_colorsを呼び出す
  useEffect(() => {
    const timer = setInterval(async () => {
      try {
        const point = await invoke("get_point") as { x: number; y: number };

        const p1 = { x: point.x - pickMargin, y: point.y - pickMargin };
        const p2 = { x: point.x + pickMargin, y: point.y + pickMargin };
        const res = await invoke("pick_colors", { p1, p2 });

        setColors(res as any[]);
      } catch (e) {
        setColors([]);
      }
    }, 50);
    return () => {
      clearInterval(timer);
    };

  }, [pickMargin]);

  return (
    <ColorPickerProvider>
      <main className="flex flex-col justify-center items-center">
        <h1 className="text-center">Welcome to Tauri + React</h1>

        <div className="flex justify-center items-center">
          <a href="https://vitejs.dev" target="_blank" rel="noreferrer">
            <img src="/vite.svg" className="logo vite" alt="Vite logo" />
          </a>
          <a href="https://tauri.app" target="_blank" rel="noreferrer">
            <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
          </a>
          <a href="https://reactjs.org" target="_blank" rel="noreferrer">
            <img src={reactLogo} className="logo react" alt="React logo" />
          </a>
        </div>
        <p>Click on the Tauri, Vite, and React logos to learn more.</p>

        <form
          className="flex justify-center"
          onSubmit={(e) => {
            e.preventDefault();
            greet();
          }}
        >
          <input
            id="greet-input"
            className="input"
            onChange={(e) => setName(e.currentTarget.value)}
            placeholder="Enter a name..."
          />
          <button className="btn" type="submit">
            Greet
          </button>
        </form>
        <p className="mt-4">{greetMsg}</p>

        <div className="flex flex-col items-center mt-4">
          <button className="btn" onClick={getPoint}>
            Get Point
          </button>
          <p className="mt-2">
            Point: {point.x}, {point.y}
          </p>
        </div>

        <div className="flex flex-col items-center mt-4">
          <p>pick_colors response ({colors.length} colors):</p>

          <div
            className="w-30 h-30"
          >
            <ColorMatrix
              colors={colors}
              gap="2px"
            />
          </div>
          {Boolean(getPointError) && (
            <p className="text-red-500 mt-2">
              Error: {getPointError}
            </p>
          )}

          <Inner />

        </div>
      </main>
    </ColorPickerProvider>
  );
}

function Inner() {
  const context = useColorPicker();
  const { watching, startWatching, stopWatching, error } = context;

  return (
    <div>
      {error && <p className="text-red-500">{error}</p>}
      <button className="btn" onClick={watching ? stopWatching : startWatching}>
        {watching ? "Stop Watching" : "Start Watching"}
      </button>
    </div>
  )

}

export default App;
