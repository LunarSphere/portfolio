use ratzilla::{
    event::{KeyCode, KeyEvent, MouseButton, MouseEvent, MouseEventKind},
    ratatui::layout::Rect,
};

use crate::{
    browser,
    command::{self, Command},
    data::{self, Project, SocialLink},
    theme::Theme,
};

const MAX_OUTPUT_HISTORY: usize = 8; // max # of outputs that can show at a time?

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Panel {
    Welcome,
    Help,
    About,
    Projects,
    Socials,
    Resume,
}

// swaps
impl Panel {
    pub fn title(self) -> &'static str {
        match self {
            Self::Welcome => "welcome",
            Self::Help => "help",
            Self::About => "about",
            Self::Projects => "projects",
            Self::Socials => "socials",
            Self::Resume => "resume",
        }
    }
}

#[derive(Debug, Clone)]
pub struct OutputEntry {
    pub command: String,
    pub lines: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct HitZone {
    pub panel: Panel,
    pub index: usize,
    pub area: Rect,
}

#[derive(Debug)]
pub struct App {
    pub active_panel: Panel,
    pub command_input: String,
    pub command_history: Vec<String>,
    pub output_history: Vec<OutputEntry>,
    pub theme: Theme,
    pub selected_project_index: usize,
    pub selected_social_index: usize,
    pub project_scroll: usize,
    pub social_scroll: usize,
    pub last_status: String,
    pub projects: Vec<Project>,
    pub socials: Vec<SocialLink>,
    hit_zones: Vec<HitZone>,
}

impl App {
    pub fn new() -> Self {
        Self {
            active_panel: Panel::Welcome,
            command_input: String::new(),
            command_history: Vec::new(),
            output_history: vec![OutputEntry {
                command: "system".to_string(),
                lines: vec![
                    "Portfolio shell ready.".to_string(),
                    "Type /help to see available commands.".to_string(),
                ],
            }],
            theme: Theme::Dark,
            selected_project_index: 0,
            selected_social_index: 0,
            project_scroll: 0,
            social_scroll: 0,
            last_status: "ready | /help".to_string(),
            projects: data::load_projects(),
            socials: data::load_socials(),
            hit_zones: Vec::new(),
        }
    }
    // defines how to handle different key inputs
    pub fn handle_key_event(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('l') if key.ctrl => self.clear(),
            KeyCode::Char('k') if key.ctrl => self.command_input.clear(),
            KeyCode::Char(ch) if !key.ctrl && !key.alt => self.command_input.push(ch),
            KeyCode::Backspace => {
                self.command_input.pop();
            }
            KeyCode::Enter => self.handle_enter(),
            KeyCode::Up => self.move_selection(-1),
            KeyCode::Down => self.move_selection(1),
            KeyCode::Esc => self.handle_escape(),
            _ => {}
        }
    }
    // move zone to where mouse hits or do nothing if same zone
    pub fn handle_mouse_event(&mut self, mouse: MouseEvent) {
        let Some(zone) = self.hit_zone_at(mouse.col, mouse.row).cloned() else {
            return;
        };

        if zone.panel != self.active_panel {
            return;
        }

        match mouse.kind {
            MouseEventKind::Moved | MouseEventKind::Entered => self.select(zone.panel, zone.index),
            MouseEventKind::SingleClick(MouseButton::Left)
            | MouseEventKind::DoubleClick(MouseButton::Left) => {
                self.select(zone.panel, zone.index);
                self.open_selected();
            }
            _ => {}
        }
    }
    // clear what zone we are in
    pub fn clear_hit_zones(&mut self) {
        self.hit_zones.clear();
    }

    pub fn add_hit_zone(&mut self, panel: Panel, index: usize, area: Rect) {
        self.hit_zones.push(HitZone { panel, index, area });
    }

    pub fn ensure_project_visible(&mut self, visible_rows: usize) {
        self.project_scroll = ensure_visible(
            self.selected_project_index,
            self.project_scroll,
            visible_rows,
        );
    }

    pub fn ensure_social_visible(&mut self, visible_rows: usize) {
        self.social_scroll =
            ensure_visible(self.selected_social_index, self.social_scroll, visible_rows);
    }

    pub fn selected_project(&self) -> Option<&Project> {
        self.projects.get(self.selected_project_index)
    }

    pub fn selected_social(&self) -> Option<&SocialLink> {
        self.socials.get(self.selected_social_index)
    }

    fn handle_enter(&mut self) {
        if self.command_input.trim().is_empty() {
            self.open_selected();
            return;
        }

        let input = std::mem::take(&mut self.command_input);
        self.run_command(input);
    }

    fn handle_escape(&mut self) {
        if self.command_input.is_empty() {
            self.active_panel = Panel::Welcome;
            self.last_status = "welcome | /help".to_string();
        } else {
            self.command_input.clear();
            self.last_status = "input cleared".to_string();
        }
    }

    fn run_command(&mut self, input: String) {
        let parsed = command::parse(&input);
        if !matches!(parsed, Command::Noop) {
            self.command_history.push(input.clone());
        }

        match parsed {
            Command::Noop => {}
            Command::Help => {
                self.active_panel = Panel::Help;
                self.push_output(input, ["Opened help.".to_string()]);
                self.last_status = "help | Enter opens selected links".to_string();
            }
            Command::About => {
                self.active_panel = Panel::About;
                self.push_output(input, ["Opened about.".to_string()]);
                self.last_status = "about".to_string();
            }
            Command::Projects => {
                self.active_panel = Panel::Projects;
                self.selected_project_index = self
                    .selected_project_index
                    .min(self.projects.len().saturating_sub(1));
                self.push_output(
                    input,
                    [
                        "Browsing projects.".to_string(),
                        "Use Up/Down or hover; Enter opens the selected URL.".to_string(),
                    ],
                );
                self.last_status = "projects | Up/Down | Enter".to_string();
            }
            Command::Resume => {
                self.active_panel = Panel::Resume;
                let open_result = browser::open_resume();
                let mut lines = vec![
                    "Starting resume open request...".to_string(),
                    format!("Resume available at {}", browser::RESUME_PATH),
                ];
                match open_result {
                    Ok(()) => self.last_status = "resume requested".to_string(),
                    Err(error) => {
                        self.last_status = "resume link shown".to_string();
                        lines.push(format!("Browser request failed: {error}"));
                    }
                }
                self.push_output(input, lines);
            }
            Command::Socials => {
                self.active_panel = Panel::Socials;
                self.selected_social_index = self
                    .selected_social_index
                    .min(self.socials.len().saturating_sub(1));
                self.push_output(
                    input,
                    [
                        "Browsing socials.".to_string(),
                        "Use Up/Down or hover; Enter opens the selected link.".to_string(),
                    ],
                );
                self.last_status = "socials | Up/Down | Enter".to_string();
            }
            Command::Toggle => {
                self.theme = self.theme.toggle();
                browser::apply_theme(self.theme);
                self.last_status = format!("theme: {}", self.theme.name());
                self.push_output(input, [format!("Theme switched to {}.", self.theme.name())]);
            }
            Command::Clear => self.clear(),
            Command::Unknown(command) => {
                self.push_output(
                    input,
                    [
                        format!("Command not found: {command}"),
                        "Type /help to see available commands.".to_string(),
                    ],
                );
                self.last_status = "unknown command".to_string();
            }
        }
    }

    fn clear(&mut self) {
        self.output_history.clear();
        self.command_history.clear();
        self.active_panel = Panel::Welcome;
        self.last_status = "cleared | /help".to_string();
    }

    fn push_output<I>(&mut self, command: String, lines: I)
    where
        I: IntoIterator<Item = String>,
    {
        self.output_history.push(OutputEntry {
            command,
            lines: lines.into_iter().collect(),
        });
        let overflow = self.output_history.len().saturating_sub(MAX_OUTPUT_HISTORY);
        if overflow > 0 {
            self.output_history.drain(0..overflow);
        }
    }

    fn move_selection(&mut self, delta: isize) {
        match self.active_panel {
            Panel::Projects => {
                self.selected_project_index =
                    move_index(self.selected_project_index, self.projects.len(), delta);
                self.last_status = selected_status("project", self.selected_project_index);
            }
            Panel::Socials => {
                self.selected_social_index =
                    move_index(self.selected_social_index, self.socials.len(), delta);
                self.last_status = selected_status("social", self.selected_social_index);
            }
            _ => {}
        }
    }

    fn select(&mut self, panel: Panel, index: usize) {
        match panel {
            Panel::Projects if index < self.projects.len() => {
                self.selected_project_index = index;
                self.last_status = selected_status("project", index);
            }
            Panel::Socials if index < self.socials.len() => {
                self.selected_social_index = index;
                self.last_status = selected_status("social", index);
            }
            _ => {}
        }
    }

    fn open_selected(&mut self) {
        let url = match self.active_panel {
            Panel::Projects => self.selected_project().and_then(Project::primary_url),
            Panel::Socials => self.selected_social().and_then(SocialLink::primary_url),
            Panel::Resume => Some(browser::RESUME_PATH),
            _ => None,
        };

        let Some(url) = url.map(str::to_string) else {
            self.last_status = "no URL configured for selection".to_string();
            return;
        };

        match browser::open_external(&url) {
            Ok(()) => self.last_status = format!("opened {url}"),
            Err(error) => self.last_status = format!("open failed: {error}"),
        }
    }

    fn hit_zone_at(&self, col: u16, row: u16) -> Option<&HitZone> {
        self.hit_zones.iter().find(|zone| {
            let x_end = zone.area.x.saturating_add(zone.area.width);
            let y_end = zone.area.y.saturating_add(zone.area.height);
            col >= zone.area.x && col < x_end && row >= zone.area.y && row < y_end
        })
    }
}

fn move_index(current: usize, len: usize, delta: isize) -> usize {
    if len == 0 {
        return 0;
    }

    current
        .saturating_add_signed(delta)
        .min(len.saturating_sub(1))
}

fn ensure_visible(selected: usize, scroll: usize, visible_rows: usize) -> usize {
    if visible_rows == 0 || selected < scroll {
        selected
    } else if selected >= scroll.saturating_add(visible_rows) {
        selected.saturating_sub(visible_rows.saturating_sub(1))
    } else {
        scroll
    }
}

fn selected_status(kind: &str, index: usize) -> String {
    format!("selected {kind} {}", index + 1)
}

//tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selection_does_not_underflow_or_overflow() {
        assert_eq!(move_index(0, 3, -1), 0);
        assert_eq!(move_index(1, 3, 1), 2);
        assert_eq!(move_index(2, 3, 1), 2);
        assert_eq!(move_index(0, 0, 1), 0);
    }

    #[test]
    fn scroll_keeps_selection_visible() {
        assert_eq!(ensure_visible(0, 0, 4), 0);
        assert_eq!(ensure_visible(6, 0, 4), 3);
        assert_eq!(ensure_visible(2, 5, 4), 2);
    }
}
