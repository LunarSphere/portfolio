import type { RefObject } from "react";
import type { TextInput } from "react-native";

import type { PanelId } from "../content/types";

export interface TerminalKeyboardActions {
  panel: PanelId;
  input: string;
  inputRef: RefObject<TextInput | null>;
  setInput(value: string): void;
  submit(): void;
  escape(): void;
  move(delta: -1 | 1): void;
  clear(): void;
}

export function useTerminalKeyboard(_actions: TerminalKeyboardActions) {
  // Native platforms use the TextInput and on-screen controls directly.
}
