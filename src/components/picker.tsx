import { createContext, useCallback, useContext, useState } from "react";
import { startMouseHook, stopMouseHook } from "../ipc";

export type ColorPickerContextType = {
  watching: boolean;
  error: string | null;
  startWatching: () => Promise<void>;
  stopWatching: () => Promise<void>;
}

export type ColorPickerProviderProps = {
  children: React.ReactNode;
  onStartWatching?: () => void;
  onStopWatching?: () => void;
}


export const ColorPickerContext = createContext<ColorPickerContextType | null>(null);

export const ColorPickerProvider = (props: ColorPickerProviderProps) => {

  const { children, onStartWatching, onStopWatching } = props;
  const [watching, setWatching] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const startWatching = useCallback(async () => {

    if (watching) {
      console.warn("Already watching for color picking.");
      return;
    }

    try {
      const started = await startMouseHook();
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
