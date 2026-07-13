import Head from "expo-router/head";
import { StatusBar } from "expo-status-bar";
import { useMemo, useRef } from "react";
import {
  Pressable,
  ScrollView,
  StyleSheet,
  Text,
  TextInput,
  View,
  useWindowDimensions,
} from "react-native";

import { primaryProjectUrl, site } from "../content/site";
import type { PanelId, Project, SocialLink } from "../content/types";
import { commands } from "../core/command";
import { usePortfolio } from "../core/portfolio-context";
import { palettes, type Palette } from "../core/theme";
import { useTerminalKeyboard } from "../hooks/use-terminal-keyboard";
import { LifeBackground } from "./life-background";
import {
  LabelValue,
  monoFont,
  TerminalButton,
  TerminalFrame,
  TerminalText,
} from "./terminal-primitives";

const shortcutCommands = [
  "/projects",
  "/resume",
  "/socials",
  "/about",
  "/help",
  "/toggle",
] as const;

const banner = String.raw`  _  _______   _____ _   _ ___   _____ ___ ___ ___ ___ _    ___
 | |/ / __\ \ / /_ _| | | / __| |_   _| _ \_ _| _ ) _ ) |  | __|
 | ' <| _| \ V / | || |_| \__ \   | | |   /| || _ \ _ \ |__| _|
 |_|\_\___| \_/ |___|\___/|___/   |_| |_|_\___|___/___/____|___|`;

export function PortfolioScreen({ panel }: { panel: PanelId }) {
  const { width, height } = useWindowDimensions();
  const isMobile = width <= 700 || height <= 520;
  const { state, setInput, executeCommand, clearInput, goHome, moveSelection, openSelected } =
    usePortfolio();
  const palette = palettes[state.theme];
  const inputRef = useRef<TextInput>(null);

  const submit = () => {
    if (state.commandInput.trim()) executeCommand(state.commandInput);
    else void openSelected(panel);
  };
  const escape = () => {
    if (state.commandInput) clearInput();
    else goHome();
  };
  const keyboardActions = useMemo(
    () => ({
      panel,
      input: state.commandInput,
      inputRef,
      setInput,
      submit,
      escape,
      move: (delta: -1 | 1) => moveSelection(panel, delta),
      clear: () => executeCommand("/clear"),
    }),
    [executeCommand, panel, setInput, state.commandInput],
  );
  useTerminalKeyboard(keyboardActions);

  const pageTitle =
    panel === "welcome"
      ? site.seo.title
      : `${panel[0]?.toUpperCase()}${panel.slice(1)} | ${site.identity.shortName}`;

  return (
    <View
      testID="portfolio-root"
      style={[styles.page, { backgroundColor: palette.background }]}
    >
      <Head>
        <title>{pageTitle}</title>
        <meta name="description" content={site.seo.description} />
        <link rel="canonical" href={`${site.seo.siteUrl}${panel === "welcome" ? "" : `/${panel}`}`} />
      </Head>
      <StatusBar style={state.theme === "dark" ? "light" : "dark"} />
      <LifeBackground color={palette.backgroundLife} subtle={isMobile} />

      <View
        testID="terminal-shell"
        style={[
          styles.shellBackdrop,
          {
            backgroundColor: palette.backgroundPanel,
            borderColor: palette.accent,
            height: isMobile ? "100%" : "86%",
            width: isMobile ? "100%" : "88%",
          },
        ]}
      >
        {!isMobile ? (
          <TerminalText
            color={palette.accent}
            style={[styles.builtWith, { backgroundColor: palette.backgroundPanel }]}
          >
            {" | built with Expo | "}
          </TerminalText>
        ) : null}
        <View
          style={[
            styles.shell,
            { backgroundColor: palette.backgroundPanel, borderColor: palette.border },
          ]}
        >
          <TerminalText
            color={palette.muted}
            style={[styles.shellTitle, { backgroundColor: palette.backgroundPanel }]}
          >
            {` ${site.identity.shellUser} `}
          </TerminalText>

          <View style={styles.shellContent}>
            <Header panel={panel} palette={palette} isMobile={isMobile} theme={state.theme} />
            <View style={styles.activePanel}>
              <ActivePanel panel={panel} palette={palette} isMobile={isMobile} width={width} />
            </View>
            {isMobile ? (
              <Shortcuts panel={panel} palette={palette} />
            ) : (
              <History palette={palette} />
            )}
            <Prompt palette={palette} isMobile={isMobile} inputRef={inputRef} submit={submit} />
          </View>

          <TerminalText
            color={palette.muted}
            numberOfLines={1}
            style={[styles.shellStatus, { backgroundColor: palette.backgroundPanel }]}
          >
            {` ${state.lastStatus} `}
          </TerminalText>
        </View>
      </View>
    </View>
  );
}

function Header({
  panel,
  palette,
  isMobile,
  theme,
}: {
  panel: PanelId;
  palette: Palette;
  isMobile: boolean;
  theme: "dark" | "light";
}) {
  if (isMobile) {
    return (
      <TerminalText color={palette.foreground} style={styles.mobileHeader} numberOfLines={1}>
        <Text style={{ color: palette.accent, fontWeight: "700" }}>{site.identity.shortName}</Text>
        {` | ${panel}`}
      </TerminalText>
    );
  }

  return (
    <View style={styles.desktopHeader}>
      <TerminalText color={palette.accent} style={styles.banner}>
        {banner}
      </TerminalText>
      <TerminalText color={palette.foreground} style={styles.centerText}>
        {site.identity.tagline}
      </TerminalText>
      <TerminalText color={palette.foreground} style={styles.centerText}>
        <Text style={{ color: palette.muted }}>panel: </Text>
        {panel}
        {" | "}
        <Text style={{ color: palette.muted }}>theme: </Text>
        {theme}
        {" | "}
        <Text style={{ color: palette.muted }}>hint: </Text>
        /help
      </TerminalText>
    </View>
  );
}

function ActivePanel({
  panel,
  palette,
  isMobile,
  width,
}: {
  panel: PanelId;
  palette: Palette;
  isMobile: boolean;
  width: number;
}) {
  switch (panel) {
    case "welcome":
      return <WelcomePanel palette={palette} isMobile={isMobile} />;
    case "help":
      return <HelpPanel palette={palette} isMobile={isMobile} />;
    case "about":
      return <AboutPanel palette={palette} />;
    case "resume":
      return <ResumePanel palette={palette} />;
    case "projects":
      return <ProjectsPanel palette={palette} isMobile={isMobile} width={width} />;
    case "socials":
      return <SocialsPanel palette={palette} isMobile={isMobile} width={width} />;
  }
}

function WelcomePanel({ palette, isMobile }: { palette: Palette; isMobile: boolean }) {
  return (
    <TerminalFrame title=" welcome " palette={palette} style={styles.fill}>
      <TerminalText color={palette.accent} style={styles.heading}>
        {site.welcome.heading}
      </TerminalText>
      <TerminalText color={palette.foreground}>{site.welcome.introduction}</TerminalText>
      {!isMobile ? (
        <TerminalText color={palette.foreground} style={styles.paragraphGap}>
          <Text style={{ color: palette.muted }}>Fast path: </Text>
          /projects  /resume  /socials  /help  /clear
        </TerminalText>
      ) : null}
      <TerminalText color={palette.foreground} style={styles.paragraphGap}>
        <Text style={{ color: palette.muted }}>Loaded: </Text>
        {`${site.projects.length} projects, ${site.socials.length} socials`}
      </TerminalText>
      <TerminalText color={palette.foreground} style={styles.paragraphGap}>
        {isMobile
          ? "Tap a shortcut below or type a command."
          : "Type a command in the prompt and press Enter."}
      </TerminalText>
    </TerminalFrame>
  );
}

function HelpPanel({ palette, isMobile }: { palette: Palette; isMobile: boolean }) {
  return (
    <TerminalFrame title=" help " palette={palette} style={styles.fill}>
      <ScrollView contentContainerStyle={styles.scrollContent}>
        <TerminalText color={palette.accent} style={styles.heading}>Commands</TerminalText>
        {commands.map((command) => (
          <View key={command.name} style={styles.helpRow}>
            <TerminalText color={palette.foreground} style={styles.helpCommand}>
              {command.name}
            </TerminalText>
            <TerminalText color={palette.muted} style={styles.helpDescription}>
              {command.description}
            </TerminalText>
          </View>
        ))}
        <TerminalText color={palette.foreground} style={styles.paragraphGap}>
          <Text style={{ color: palette.muted }}>Lists: </Text>
          {isMobile
            ? "tap an item to open its primary link."
            : "Up/Down or hover selects; Enter opens the selected primary URL."}
        </TerminalText>
        <TerminalText color={palette.foreground}>
          <Text style={{ color: palette.muted }}>Escape: </Text>
          clear input or return to welcome.
        </TerminalText>
      </ScrollView>
    </TerminalFrame>
  );
}

function AboutPanel({ palette }: { palette: Palette }) {
  return (
    <TerminalFrame title=" about " palette={palette} style={styles.fill}>
      <TerminalText color={palette.accent} style={styles.heading}>{site.about.heading}</TerminalText>
      {site.about.paragraphs.map((paragraph) => (
        <TerminalText key={paragraph} color={palette.foreground}>{paragraph}</TerminalText>
      ))}
      <TerminalText color={palette.foreground} style={styles.paragraphGap}>
        <Text style={{ color: palette.muted }}>Next: </Text>
        /projects for work samples, /resume for the PDF, /socials for links.
      </TerminalText>
    </TerminalFrame>
  );
}

function ResumePanel({ palette }: { palette: Palette }) {
  const { openUrl } = usePortfolio();
  return (
    <TerminalFrame title=" resume " palette={palette} style={styles.fill}>
      <TerminalText color={palette.accent} style={styles.heading}>{site.resume.heading}</TerminalText>
      <TerminalText color={palette.foreground}>{site.resume.description}</TerminalText>
      <TerminalButton
        accessibilityLabel="Open resume PDF"
        label={`[open ${site.resume.path}]`}
        palette={palette}
        onPress={() => void openUrl(site.resume.path)}
        style={styles.resumeButton}
      />
      <LabelValue label="Direct path" palette={palette}>{site.resume.path}</LabelValue>
      <TerminalText color={palette.muted} style={styles.paragraphGap}>
        {site.resume.editorHint}
      </TerminalText>
    </TerminalFrame>
  );
}

function ProjectsPanel({
  palette,
  isMobile,
  width,
}: {
  palette: Palette;
  isMobile: boolean;
  width: number;
}) {
  const { state, select, selectAndOpen } = usePortfolio();
  const selected = site.projects[state.selectedProjectIndex];
  const showSideBySide = !isMobile && width >= 920;
  return (
    <View style={[styles.browser, showSideBySide ? styles.browserRow : styles.browserColumn]}>
      <TerminalFrame
        title=" projects "
        palette={palette}
        style={[styles.listFrame, showSideBySide ? styles.listNarrow : null]}
        contentStyle={styles.listContent}
      >
        <ScrollView>
          {site.projects.map((project, index) => (
            <ListItem
              key={project.slug}
              title={project.title}
              meta={project.technologies.join(", ")}
              marker={project.featured ? "*" : undefined}
              selected={index === state.selectedProjectIndex}
              palette={palette}
              onHover={() => select("projects", index)}
              onPress={() => void selectAndOpen("projects", index)}
            />
          ))}
        </ScrollView>
      </TerminalFrame>
      {!isMobile ? <ProjectPreview project={selected} palette={palette} /> : null}
    </View>
  );
}

function SocialsPanel({
  palette,
  isMobile,
  width,
}: {
  palette: Palette;
  isMobile: boolean;
  width: number;
}) {
  const { state, select, selectAndOpen } = usePortfolio();
  const selected = site.socials[state.selectedSocialIndex];
  const showSideBySide = !isMobile && width >= 920;
  return (
    <View style={[styles.browser, showSideBySide ? styles.browserRow : styles.browserColumn]}>
      <TerminalFrame
        title=" socials "
        palette={palette}
        style={[styles.listFrame, showSideBySide ? styles.listNarrow : null]}
        contentStyle={styles.listContent}
      >
        <ScrollView>
          {site.socials.map((social, index) => (
            <ListItem
              key={social.label}
              title={social.label}
              meta={social.handle ?? ""}
              selected={index === state.selectedSocialIndex}
              palette={palette}
              onHover={() => select("socials", index)}
              onPress={() => void selectAndOpen("socials", index)}
            />
          ))}
        </ScrollView>
      </TerminalFrame>
      {!isMobile ? <SocialPreview social={selected} palette={palette} /> : null}
    </View>
  );
}

function ListItem({
  title,
  meta,
  marker,
  selected,
  palette,
  onHover,
  onPress,
}: {
  title: string;
  meta: string;
  marker?: string;
  selected: boolean;
  palette: Palette;
  onHover(): void;
  onPress(): void;
}) {
  return (
    <Pressable
      accessibilityRole="link"
      accessibilityLabel={`Open ${title}`}
      onHoverIn={onHover}
      onPress={onPress}
      style={({ pressed }) => [
        styles.listItem,
        selected || pressed ? { backgroundColor: palette.selectionBackground } : null,
      ]}
    >
      <TerminalText color={selected ? palette.selectionForeground : palette.foreground}>
        <Text style={{ color: palette.accent }}>{selected ? ">" : " "}</Text>
        {" "}
        <Text style={{ color: palette.warning }}>{marker ?? " "}</Text>
        {` ${title}`}
      </TerminalText>
      <TerminalText color={palette.muted} numberOfLines={1} style={styles.listMeta}>
        {meta}
      </TerminalText>
    </Pressable>
  );
}

function ProjectPreview({ project, palette }: { project?: Project; palette: Palette }) {
  return (
    <TerminalFrame title=" preview " palette={palette} style={styles.previewFrame}>
      {project ? (
        <ScrollView contentContainerStyle={styles.scrollContent}>
          <TerminalText color={palette.accent} style={styles.heading}>{project.title}</TerminalText>
          <TerminalText color={palette.foreground}>{project.blurb}</TerminalText>
          <TerminalText color={palette.foreground} style={styles.paragraphGap}>{project.description}</TerminalText>
          <LabelValue label="Tech" palette={palette}>{project.technologies.join(", ")}</LabelValue>
          {project.liveUrl ? <LabelValue label="Live" palette={palette}>{project.liveUrl}</LabelValue> : null}
          {project.githubUrl ? <LabelValue label="GitHub" palette={palette}>{project.githubUrl}</LabelValue> : null}
          {!primaryProjectUrl(project) ? (
            <TerminalText color={palette.warning}>No URL configured yet.</TerminalText>
          ) : null}
        </ScrollView>
      ) : (
        <TerminalText color={palette.muted}>No project selected.</TerminalText>
      )}
    </TerminalFrame>
  );
}

function SocialPreview({ social, palette }: { social?: SocialLink; palette: Palette }) {
  return (
    <TerminalFrame title=" preview " palette={palette} style={styles.previewFrame}>
      {social ? (
        <>
          <TerminalText color={palette.accent} style={styles.heading}>{social.label}</TerminalText>
          <TerminalText color={palette.muted}>{social.handle}</TerminalText>
          <TerminalText color={palette.foreground} style={styles.paragraphGap}>{social.blurb}</TerminalText>
          <LabelValue label="URL" palette={palette}>{social.url}</LabelValue>
        </>
      ) : (
        <TerminalText color={palette.muted}>No social selected.</TerminalText>
      )}
    </TerminalFrame>
  );
}

function Shortcuts({ panel, palette }: { panel: PanelId; palette: Palette }) {
  const { executeCommand } = usePortfolio();
  return (
    <TerminalFrame
      title=" shortcuts "
      palette={palette}
      style={styles.shortcutsFrame}
      contentStyle={styles.shortcuts}
    >
      {shortcutCommands.map((command) => (
        <TerminalButton
          key={command}
          label={`[${command}]`}
          palette={palette}
          active={command === `/${panel}`}
          onPress={() => executeCommand(command)}
        />
      ))}
    </TerminalFrame>
  );
}

function History({ palette }: { palette: Palette }) {
  const { state } = usePortfolio();
  const entries = [...state.outputHistory].reverse();
  return (
    <TerminalFrame title=" output " palette={palette} style={styles.historyFrame}>
      <ScrollView>
        {entries.length ? (
          entries.map((entry, index) => (
            <View key={`${entry.command}-${index}`}>
              <TerminalText color={palette.foreground}>
                <Text style={{ color: palette.accent }}>$ </Text>
                {entry.command}
              </TerminalText>
              {entry.lines.map((line) => (
                <TerminalText key={line} color={palette.muted}>{`  ${line}`}</TerminalText>
              ))}
            </View>
          ))
        ) : (
          <TerminalText color={palette.muted}>No output. Type /help.</TerminalText>
        )}
      </ScrollView>
    </TerminalFrame>
  );
}

function Prompt({
  palette,
  isMobile,
  inputRef,
  submit,
}: {
  palette: Palette;
  isMobile: boolean;
  inputRef: React.RefObject<TextInput | null>;
  submit(): void;
}) {
  const { state, setInput } = usePortfolio();
  return (
    <TerminalFrame
      title={isMobile ? " command optional " : " command "}
      palette={palette}
      style={styles.promptFrame}
      contentStyle={styles.promptContent}
    >
      <TerminalText color={palette.foreground}>{`${site.identity.shellUser}:~$ `}</TerminalText>
      <TextInput
        ref={inputRef}
        accessibilityLabel="Terminal command"
        autoCapitalize="none"
        autoCorrect={false}
        blurOnSubmit={false}
        onChangeText={setInput}
        onSubmitEditing={submit}
        placeholder="_"
        placeholderTextColor={palette.muted}
        returnKeyType="go"
        selectionColor={palette.accent}
        style={[styles.input, { color: palette.foreground }]}
        value={state.commandInput}
      />
    </TerminalFrame>
  );
}

const styles = StyleSheet.create({
  page: {
    alignItems: "center",
    flex: 1,
    justifyContent: "center",
    minHeight: 0,
    overflow: "hidden",
  },
  shellBackdrop: {
    borderRadius: 8,
    borderWidth: 1,
    padding: 7,
    position: "relative",
  },
  shell: {
    borderColor: "transparent",
    borderRadius: 7,
    borderWidth: 1,
    flex: 1,
    minHeight: 0,
    padding: 8,
    position: "relative",
  },
  shellContent: {
    flex: 1,
    gap: 12,
    minHeight: 0,
    paddingBottom: 2,
    paddingTop: 3,
  },
  shellTitle: {
    left: 12,
    paddingHorizontal: 4,
    position: "absolute",
    top: -11,
    zIndex: 3,
  },
  shellStatus: {
    bottom: -11,
    left: 12,
    maxWidth: "80%",
    paddingHorizontal: 4,
    position: "absolute",
    zIndex: 3,
  },
  builtWith: {
    bottom: -11,
    paddingHorizontal: 4,
    position: "absolute",
    right: 16,
    zIndex: 4,
  },
  desktopHeader: {
    height: 130,
    justifyContent: "center",
  },
  mobileHeader: {
    height: 24,
    paddingHorizontal: 4,
  },
  banner: {
    fontSize: 12,
    lineHeight: 15,
    textAlign: "center",
  },
  centerText: {
    textAlign: "center",
  },
  activePanel: {
    flex: 1,
    minHeight: 0,
  },
  fill: {
    flex: 1,
  },
  heading: {
    fontWeight: "700",
    marginBottom: 5,
  },
  paragraphGap: {
    marginTop: 16,
  },
  scrollContent: {
    paddingBottom: 8,
  },
  helpRow: {
    flexDirection: "row",
    gap: 12,
  },
  helpCommand: {
    fontWeight: "700",
    width: 100,
  },
  helpDescription: {
    flex: 1,
  },
  resumeButton: {
    alignSelf: "flex-start",
    marginVertical: 16,
  },
  browser: {
    flex: 1,
    gap: 12,
    minHeight: 0,
  },
  browserRow: {
    flexDirection: "row",
  },
  browserColumn: {
    flexDirection: "column",
  },
  listFrame: {
    flex: 1,
  },
  listNarrow: {
    flexBasis: "38%",
    flexGrow: 0,
  },
  listContent: {
    paddingHorizontal: 3,
    paddingVertical: 8,
  },
  listItem: {
    minHeight: 54,
    paddingHorizontal: 6,
    paddingVertical: 6,
  },
  listMeta: {
    paddingLeft: 28,
  },
  previewFrame: {
    flex: 1,
  },
  shortcutsFrame: {
    minHeight: 88,
  },
  shortcuts: {
    alignContent: "center",
    flexDirection: "row",
    flexWrap: "wrap",
    paddingHorizontal: 5,
    paddingVertical: 7,
  },
  historyFrame: {
    height: 112,
  },
  promptFrame: {
    height: 56,
  },
  promptContent: {
    alignItems: "center",
    flexDirection: "row",
    paddingHorizontal: 8,
    paddingVertical: 7,
  },
  input: {
    flex: 1,
    fontFamily: monoFont,
    fontSize: 14,
    height: 28,
    lineHeight: 20,
    padding: 0,
  },
});
