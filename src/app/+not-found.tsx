import { Link } from "expo-router";
import { StyleSheet, Text, View } from "react-native";

import { palettes } from "../core/theme";

export default function NotFoundRoute() {
  const palette = palettes.dark;
  return (
    <View style={[styles.page, { backgroundColor: palette.background }]}>
      <Text style={[styles.text, { color: palette.accent }]}>404 | command not found</Text>
      <Link href="/" style={[styles.link, { color: palette.foreground }]}>
        Return to guest@portfolio
      </Link>
    </View>
  );
}

const styles = StyleSheet.create({
  page: {
    alignItems: "center",
    flex: 1,
    gap: 18,
    justifyContent: "center",
  },
  text: {
    fontFamily: "monospace",
    fontSize: 18,
  },
  link: {
    fontFamily: "monospace",
    fontSize: 14,
  },
});
