import type { PanelId } from "../content/types";

export type CommandId =
  | "noop"
  | "help"
  | "about"
  | "projects"
  | "resume"
  | "socials"
  | "toggle"
  | "clear"
  | "unknown";

export interface ParsedCommand {
  id: CommandId;
  raw: string;
}

export interface CommandSpec {
  name: string;
  description: string;
  panel?: PanelId;
}

export const commands: CommandSpec[] = [
  { name: "/help", description: "Show available commands.", panel: "help" },
  { name: "/about", description: "Show a concise introduction.", panel: "about" },
  { name: "/projects", description: "Browse selected projects.", panel: "projects" },
  { name: "/resume", description: "Show the resume link.", panel: "resume" },
  { name: "/socials", description: "Browse social and profile links.", panel: "socials" },
  { name: "/toggle", description: "Toggle light or dark theme." },
  { name: "/clear", description: "Clear command output." },
];

const knownCommands = new Set(commands.map((command) => command.name.slice(1)));

export function parseCommand(input: string): ParsedCommand {
  const raw = input.trim();
  if (!raw) return { id: "noop", raw };

  const token = raw.split(/\s+/u)[0] ?? "";
  const normalized = token.replace(/^\//u, "").toLowerCase();
  return {
    id: knownCommands.has(normalized) ? (normalized as CommandId) : "unknown",
    raw,
  };
}

export function panelPath(panel: PanelId): string {
  return panel === "welcome" ? "/" : `/${panel}`;
}
