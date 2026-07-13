import { useEffect } from "react";

import type { TerminalKeyboardActions } from "./use-terminal-keyboard";

export function useTerminalKeyboard(actions: TerminalKeyboardActions) {
  useEffect(() => {
    const onKeyDown = (event: KeyboardEvent) => {
      if (event.metaKey || event.altKey) return;

      if (event.ctrlKey && event.key.toLowerCase() === "l") {
        event.preventDefault();
        actions.clear();
        return;
      }
      if (event.ctrlKey && event.key.toLowerCase() === "k") {
        event.preventDefault();
        actions.setInput("");
        return;
      }
      if (event.key === "ArrowUp" || event.key === "ArrowDown") {
        event.preventDefault();
        actions.move(event.key === "ArrowUp" ? -1 : 1);
        return;
      }
      if (event.key === "Escape") {
        event.preventDefault();
        actions.escape();
        return;
      }

      const target = event.target as HTMLElement | null;
      const isPrompt = target?.getAttribute("aria-label") === "Terminal command";
      if (isPrompt) return;
      if (target?.matches("input, textarea, select, [contenteditable='true']")) return;

      if (event.key === "Enter") {
        event.preventDefault();
        actions.submit();
      } else if (event.key === "Backspace") {
        event.preventDefault();
        actions.setInput(actions.input.slice(0, -1));
        actions.inputRef.current?.focus();
      } else if (!event.ctrlKey && event.key.length === 1) {
        event.preventDefault();
        actions.setInput(actions.input + event.key);
        actions.inputRef.current?.focus();
      }
    };

    document.addEventListener("keydown", onKeyDown);
    return () => document.removeEventListener("keydown", onKeyDown);
  }, [actions]);
}
