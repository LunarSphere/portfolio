import { Slot } from "expo-router";
import { SafeAreaProvider } from "react-native-safe-area-context";

import { PortfolioProvider } from "../core/portfolio-context";
import "../global.css";

export default function RootLayout() {
  return (
    <SafeAreaProvider>
      <PortfolioProvider>
        <Slot />
      </PortfolioProvider>
    </SafeAreaProvider>
  );
}
