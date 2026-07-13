import { site } from "../content/site.ts";
import type { PanelId } from "../content/types";
import type { ThemeName } from "./theme";

export const MAX_OUTPUT_HISTORY = 8;

export interface OutputEntry {
  command: string;
  lines: string[];
}

export interface PortfolioState {
  commandInput: string;
  commandHistory: string[];
  outputHistory: OutputEntry[];
  theme: ThemeName;
  selectedProjectIndex: number;
  selectedSocialIndex: number;
  lastStatus: string;
}

export type PortfolioAction =
  | { type: "set-input"; value: string }
  | { type: "set-status"; value: string }
  | { type: "toggle-theme" }
  | { type: "clear" }
  | { type: "output"; entry: OutputEntry; record?: boolean }
  | { type: "select"; panel: "projects" | "socials"; index: number }
  | { type: "move"; panel: PanelId; delta: -1 | 1 };

export const initialPortfolioState: PortfolioState = {
  commandInput: "",
  commandHistory: [],
  outputHistory: [
    {
      command: "system",
      lines: ["Portfolio shell ready.", "Type /help to see available commands."],
    },
  ],
  theme: "dark",
  selectedProjectIndex: 0,
  selectedSocialIndex: 0,
  lastStatus: "ready | /help",
};

export function clampIndex(index: number, length: number): number {
  if (length === 0) return 0;
  return Math.max(0, Math.min(index, length - 1));
}

export function portfolioReducer(
  state: PortfolioState,
  action: PortfolioAction,
): PortfolioState {
  switch (action.type) {
    case "set-input":
      return { ...state, commandInput: action.value };
    case "set-status":
      return { ...state, lastStatus: action.value };
    case "toggle-theme":
      return { ...state, theme: state.theme === "dark" ? "light" : "dark" };
    case "clear":
      return {
        ...state,
        commandInput: "",
        commandHistory: [],
        outputHistory: [],
        lastStatus: "cleared | /help",
      };
    case "output": {
      const outputHistory = [...state.outputHistory, action.entry].slice(
        -MAX_OUTPUT_HISTORY,
      );
      return {
        ...state,
        commandInput: "",
        outputHistory,
        commandHistory: action.record
          ? [...state.commandHistory, action.entry.command]
          : state.commandHistory,
      };
    }
    case "select": {
      const length =
        action.panel === "projects" ? site.projects.length : site.socials.length;
      const index = clampIndex(action.index, length);
      return action.panel === "projects"
        ? {
            ...state,
            selectedProjectIndex: index,
            lastStatus: `selected project ${index + 1}`,
          }
        : {
            ...state,
            selectedSocialIndex: index,
            lastStatus: `selected social ${index + 1}`,
          };
    }
    case "move": {
      if (action.panel === "projects") {
        const index = clampIndex(
          state.selectedProjectIndex + action.delta,
          site.projects.length,
        );
        return {
          ...state,
          selectedProjectIndex: index,
          lastStatus: `selected project ${index + 1}`,
        };
      }
      if (action.panel === "socials") {
        const index = clampIndex(
          state.selectedSocialIndex + action.delta,
          site.socials.length,
        );
        return {
          ...state,
          selectedSocialIndex: index,
          lastStatus: `selected social ${index + 1}`,
        };
      }
      return state;
    }
  }
}
