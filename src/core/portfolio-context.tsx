import { useRouter, type Href } from "expo-router";
import {
  createContext,
  useCallback,
  useContext,
  useMemo,
  useReducer,
  type PropsWithChildren,
} from "react";

import { primaryProjectUrl, site } from "../content/site";
import type { PanelId } from "../content/types";
import { panelPath, parseCommand } from "./command";
import { openExternal } from "./open-external";
import {
  initialPortfolioState,
  portfolioReducer,
  type PortfolioState,
} from "./state";

interface PortfolioContextValue {
  state: PortfolioState;
  setInput(value: string): void;
  executeCommand(value: string): void;
  clearInput(): void;
  goHome(): void;
  moveSelection(panel: PanelId, delta: -1 | 1): void;
  select(panel: "projects" | "socials", index: number): void;
  selectAndOpen(panel: "projects" | "socials", index: number): Promise<void>;
  openSelected(panel: PanelId): Promise<void>;
  openUrl(url: string): Promise<void>;
}

const PortfolioContext = createContext<PortfolioContextValue | null>(null);

export function PortfolioProvider({ children }: PropsWithChildren) {
  const [state, dispatch] = useReducer(portfolioReducer, initialPortfolioState);
  const router = useRouter();

  const navigate = useCallback(
    (panel: PanelId) => router.push(panelPath(panel) as Href),
    [router],
  );

  const executeCommand = useCallback(
    (value: string) => {
      const parsed = parseCommand(value);
      if (parsed.id === "noop") return;
      if (parsed.id === "clear") {
        dispatch({ type: "clear" });
        navigate("welcome");
        return;
      }
      if (parsed.id === "toggle") {
        const nextTheme = state.theme === "dark" ? "light" : "dark";
        dispatch({ type: "toggle-theme" });
        dispatch({
          type: "output",
          entry: {
            command: parsed.raw,
            lines: [`Theme switched to ${nextTheme}.`],
          },
          record: true,
        });
        dispatch({ type: "set-status", value: `theme: ${nextTheme}` });
        return;
      }

      const panel =
        parsed.id === "unknown"
          ? undefined
          : (parsed.id as Exclude<PanelId, "welcome">);
      if (!panel) {
        dispatch({
          type: "output",
          entry: {
            command: parsed.raw,
            lines: [
              `Command not found: ${parsed.raw}`,
              "Type /help to see available commands.",
            ],
          },
          record: true,
        });
        dispatch({ type: "set-status", value: "unknown command" });
        return;
      }

      const output: Record<Exclude<PanelId, "welcome">, string[]> = {
        help: ["Opened help."],
        about: ["Opened about."],
        projects: ["Browsing projects.", "Use Up/Down or hover; Enter opens the selected URL."],
        socials: ["Browsing socials.", "Use Up/Down or hover; Enter opens the selected link."],
        resume: ["Opened resume panel.", `Resume available at ${site.resume.path}`],
      };
      dispatch({
        type: "output",
        entry: { command: parsed.raw, lines: output[panel] },
        record: true,
      });
      dispatch({
        type: "set-status",
        value:
          panel === "projects" || panel === "socials"
            ? `${panel} | tap or Enter`
            : panel,
      });
      navigate(panel);
    },
    [navigate, state.theme],
  );

  const openUrl = useCallback(async (url: string) => {
    try {
      await openExternal(url);
      dispatch({ type: "set-status", value: `opened ${url}` });
    } catch (error) {
      dispatch({
        type: "set-status",
        value: `open failed: ${error instanceof Error ? error.message : String(error)}`,
      });
    }
  }, []);

  const selectedUrl = useCallback(
    (panel: PanelId, projectIndex = state.selectedProjectIndex, socialIndex = state.selectedSocialIndex) => {
      if (panel === "projects") {
        const project = site.projects[projectIndex];
        return project ? primaryProjectUrl(project) : undefined;
      }
      if (panel === "socials") return site.socials[socialIndex]?.url;
      if (panel === "resume") return site.resume.path;
      return undefined;
    },
    [state.selectedProjectIndex, state.selectedSocialIndex],
  );

  const value = useMemo<PortfolioContextValue>(
    () => ({
      state,
      setInput: (input) => dispatch({ type: "set-input", value: input }),
      executeCommand,
      clearInput: () => {
        dispatch({ type: "set-input", value: "" });
        dispatch({ type: "set-status", value: "input cleared" });
      },
      goHome: () => {
        dispatch({ type: "set-status", value: "welcome | /help" });
        navigate("welcome");
      },
      moveSelection: (panel, delta) => dispatch({ type: "move", panel, delta }),
      select: (panel, index) => dispatch({ type: "select", panel, index }),
      selectAndOpen: async (panel, index) => {
        dispatch({ type: "select", panel, index });
        const url = selectedUrl(panel, index, index);
        if (url) await openUrl(url);
        else dispatch({ type: "set-status", value: "no URL configured for selection" });
      },
      openSelected: async (panel) => {
        const url = selectedUrl(panel);
        if (url) await openUrl(url);
        else dispatch({ type: "set-status", value: "no URL configured for selection" });
      },
      openUrl,
    }),
    [executeCommand, navigate, openUrl, selectedUrl, state],
  );

  return <PortfolioContext.Provider value={value}>{children}</PortfolioContext.Provider>;
}

export function usePortfolio() {
  const context = useContext(PortfolioContext);
  if (!context) throw new Error("usePortfolio must be used inside PortfolioProvider");
  return context;
}
