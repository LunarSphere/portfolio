export type ThemeName = "dark" | "light";

export interface Palette {
  background: string;
  backgroundPanel: string;
  backgroundLife: string;
  foreground: string;
  muted: string;
  border: string;
  accent: string;
  warning: string;
  success: string;
  selectionBackground: string;
  selectionForeground: string;
}

export const palettes: Record<ThemeName, Palette> = {
  dark: {
    background: "#0b0f11",
    backgroundPanel: "#101827",
    backgroundLife: "#465259",
    foreground: "#d9e7e5",
    muted: "#8fa1a6",
    border: "#3d4c50",
    accent: "#6bd0e3",
    warning: "#f4bf75",
    success: "#9ece6a",
    selectionBackground: "#193740",
    selectionForeground: "#ebfdff",
  },
  light: {
    background: "#f4f7f8",
    backgroundPanel: "#eaf1f3",
    backgroundLife: "#aec0c5",
    foreground: "#172126",
    muted: "#53676d",
    border: "#aec0c5",
    accent: "#117285",
    warning: "#9e5b1c",
    success: "#2d7744",
    selectionBackground: "#d3edf2",
    selectionForeground: "#0d343d",
  },
};
