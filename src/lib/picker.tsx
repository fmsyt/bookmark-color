import { createContext, useCallback, useContext, useLayoutEffect, useState } from "react";
import { startMouseHook, stopMouseHook } from "../ipc";
import { listen, UnlistenFn } from "@tauri-apps/api/event";

export type PickedColor = {
  x: number;
  y: number;
  button: string; // e.g., "left", "right", "middle"
  rgb: [number, number, number] | null; // RGB color value, or null if not available
};


export type ColorPickerContextType = {
  watching: boolean;
  error: string | null;
  startWatching: () => Promise<void>;
  stopWatching: () => Promise<void>;
  pickedColor: PickedColor | null;
}

export type ColorPickerProviderProps = {
  children: React.ReactNode;
  onStartWatching?: () => void;
  onStopWatching?: () => void;
}


export const ColorPickerContext = createContext<ColorPickerContextType | null>(null);

async function openListener(onPickColor?: (event: PickedColor) => void) {
  const unlisten = await listen<PickedColor>("mouse-click", (event) => {
    console.log("Mouse clicked:", event);
    onPickColor?.(event.payload);
  });

  return unlisten;
}

export const ColorPickerProvider = (props: ColorPickerProviderProps) => {

  const { children, onStartWatching, onStopWatching } = props;
  const [watching, setWatching] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [pickedColor, setPickedColor] = useState<PickedColor | null>(null);

  const onPickColor = useCallback((color: PickedColor) => {
    console.log("Picked color:", color);
    setPickedColor(color);
  }, []);

  useLayoutEffect(() => {

    let unlisten: UnlistenFn | null = null;

    openListener(onPickColor)
    .then((listener) => {
      unlisten = listener;
    })
    .catch((err) => {
      console.error("Failed to set up mouse click listener:", err);
      setError("Failed to set up mouse click listener. Check the console for details.");
    });

    return () => {
      if (unlisten) {
        unlisten();
      }
    };
  }, [onPickColor]);

  const startWatching = useCallback(async () => {

    if (watching) {
      console.warn("Already watching for color picking.");
      return;
    }

    try {
      const started = await startMouseHook();
      if (!started) {
        console.error("Failed to start color picking.");
        setError("Failed to start color picking. Check the console for details.");
        return;
      }

      setWatching(started);

      setError(null);
      onStartWatching?.();

    } catch (error) {
      console.error("Error starting color picking:", error);
      setError("Failed to start color picking. Check the console for details.");
    }

  }, [watching, onStartWatching]);

  const stopWatching = useCallback(async () => {
    if (!watching) {
      console.warn("Not currently watching for color picking.");
      return;
    }

    try {
      const stopped = await stopMouseHook();
      if (!stopped) {
        console.error("Failed to stop color picking.");
        setError("Failed to stop color picking. Check the console for details.");
        return;
      }

      setWatching(!stopped);

      onStopWatching?.();

    } catch (error) {
      console.error("Error stopping color picking:", error);
      setError("Failed to stop color picking. Check the console for details.");
    }
  }, [watching, onStopWatching]);

  return (
    <ColorPickerContext.Provider
      value={{
        watching,
        error,
        startWatching,
        stopWatching,
        pickedColor,
      }}
    >
      {children}
    </ColorPickerContext.Provider>
  );
}

export const useColorPicker = () => {
  const context = useContext(ColorPickerContext);
  if (!context) {
    throw new Error("useColorPicker must be used within a ColorPickerProvider");
  }
  return context;
}
