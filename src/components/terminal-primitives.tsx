import type { PropsWithChildren, ReactNode } from "react";
import {
  Pressable,
  StyleSheet,
  Text,
  View,
  type PressableProps,
  type StyleProp,
  type TextStyle,
  type ViewStyle,
} from "react-native";

import type { Palette } from "../core/theme";

export const monoFont = "monospace";

export function TerminalText({
  children,
  color,
  style,
  numberOfLines,
}: PropsWithChildren<{
  color?: string;
  style?: StyleProp<TextStyle>;
  numberOfLines?: number;
}>) {
  return (
    <Text numberOfLines={numberOfLines} style={[styles.text, color ? { color } : null, style]}>
      {children}
    </Text>
  );
}

export function TerminalFrame({
  title,
  children,
  palette,
  style,
  contentStyle,
  testID,
}: PropsWithChildren<{
  title: string;
  palette: Palette;
  style?: StyleProp<ViewStyle>;
  contentStyle?: StyleProp<ViewStyle>;
  testID?: string;
}>) {
  return (
    <View
      testID={testID}
      style={[styles.frame, { borderColor: palette.border, backgroundColor: palette.background }, style]}
    >
      <TerminalText
        color={palette.muted}
        style={[styles.frameTitle, { backgroundColor: palette.background }]}
      >
        {title}
      </TerminalText>
      <View style={[styles.frameContent, contentStyle]}>{children}</View>
    </View>
  );
}

export function TerminalButton({
  label,
  palette,
  active = false,
  style,
  ...props
}: Omit<PressableProps, "children" | "style"> & {
  label: string;
  palette: Palette;
  active?: boolean;
  style?: StyleProp<ViewStyle>;
}) {
  return (
    <Pressable
      accessibilityRole="button"
      {...props}
      style={({ pressed, hovered }) => [
        styles.button,
        active || pressed || hovered
          ? { backgroundColor: palette.selectionBackground }
          : null,
        style,
      ]}
    >
      <TerminalText
        color={active ? palette.selectionForeground : palette.accent}
        style={active ? styles.bold : null}
      >
        {label}
      </TerminalText>
    </Pressable>
  );
}

export function LabelValue({
  label,
  children,
  palette,
}: {
  label: string;
  children: ReactNode;
  palette: Palette;
}) {
  return (
    <TerminalText color={palette.foreground}>
      <Text style={{ color: palette.muted }}>{label}: </Text>
      {children}
    </TerminalText>
  );
}

const styles = StyleSheet.create({
  text: {
    fontFamily: monoFont,
    fontSize: 14,
    lineHeight: 20,
  },
  frame: {
    borderWidth: 1,
    minHeight: 0,
    position: "relative",
  },
  frameTitle: {
    left: 12,
    paddingHorizontal: 5,
    position: "absolute",
    top: -11,
    zIndex: 2,
  },
  frameContent: {
    flex: 1,
    minHeight: 0,
    padding: 14,
  },
  button: {
    minHeight: 30,
    justifyContent: "center",
    paddingHorizontal: 5,
  },
  bold: {
    fontWeight: "700",
  },
});
