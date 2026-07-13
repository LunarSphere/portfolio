use ratzilla::ratatui::{
    prelude::*,
    widgets::{Block, BorderType, Paragraph, Wrap},
};

use crate::{
    app::{App, Panel},
    theme::Palette,
};

use super::panels;

const MOBILE_COMMANDS: &[&str] = &[
    "/projects",
    "/resume",
    "/socials",
    "/about",
    "/help",
    "/toggle",
];

// const BANNER: &[&str] = &[
//     r#" __   ___  _______  ___      ___  __     ____  ____   ________      ___________  _______    __     _______   _______   ___       _______  "#,
//     r#"|/"| /  ")/"     "||"  \    /"  ||" \   ("  _||_ " | /"       )    ("     _   ")/"      \  |" \   |   _  "\ |   _  "\ |"  |     /"     "| "#,
//     r#"(: |/   /(: ______) \   \  //  / ||  |  |   (  ) : |(:   \___/      )__/  \\__/|:        | ||  |  (. |_)  :)(. |_)  :)||  |    (: ______) "#,
//     r#"|    __/  \/    |    \\  \/. ./  |:  |  (:  |  | . ) \___  \           \\_ /   |_____/   ) |:  |  |:     \/ |:     \/ |:  |     \/    |   "#,
//     r#"(// _  \  // ___)_    \.    //   |.  |   \\ \__/ //   __/  \\          |.  |    //      /  |.  |  (|  _  \\ (|  _  \\  \  |___  // ___)_  "#,
//     r#"|: | \  \(:      "|    \\   /    /\  |\  /\\ __ //\  /" \   :)         \:  |   |:  __   \  /\  |\ |: |_)  :)|: |_)  :)( \_|:  \(:      "| "#,
//     r#"(__|  \__)\_______)     \__/    (__\_|_)(__________)(_______/           \__|   |__|  \___)(__\_|_)(_______/ (_______/  \_______)\_______) "#,
//     r#"                                                                                                                                          "#,
// ];
// //
// const BANNER: &[&str] = &[
//     r#"                                                                                                            "#,
//     r#"@@@  @@@ @@@@@@@@ @@@  @@@ @@@ @@@  @@@  @@@@@@    @@@@@@@ @@@@@@@  @@@ @@@@@@@  @@@@@@@  @@@      @@@@@@@@ "#,
//     r#"@@!  !@@ @@!      @@!  @@@ @@! @@!  @@@ !@@          @!!   @@!  @@@ @@! @@!  @@@ @@!  @@@ @@!      @@!      "#,
//     r#"@!@@!@!  @!!!:!   @!@  !@! !!@ @!@  !@!  !@@!!       @!!   @!@!!@!  !!@ @!@!@!@  @!@!@!@  @!!      @!!!:!   "#,
//     r#"!!: :!!  !!:       !: .:!  !!: !!:  !!!     !:!      !!:   !!: :!!  !!: !!:  !!! !!:  !!! !!:      !!:      "#,
//     r#" :   ::: : :: ::     ::    :    :.:: :  ::.: :        :     :   : : :   :: : ::  :: : ::  : ::.: : : :: ::  "#,
//     r#"                                                                                                            "#,
// ];
//
const BANNER: &[&str] = &[
    r#"  _  _______   _____ _   _ ___   _____ ___ ___ ___ ___ _    ___ "#,
    r#" | |/ / __\ \ / /_ _| | | / __| |_   _| _ \_ _| _ ) _ ) |  | __|"#,
    r#" | ' <| _| \ V / | || |_| \__ \   | | |   /| || _ \ _ \ |__| _| "#,
    r#" |_|\_\___| \_/ |___|\___/|___/   |_| |_|_\___|___/___/____|___|"#,
    r#"                                                                "#,
];

pub fn render(frame: &mut Frame<'_>, area: Rect, app: &mut App, palette: Palette) {
    let shell = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(palette.border))
        .style(
            Style::default()
                .bg(palette.background_panel)
                .fg(palette.foreground),
        )
        .title(" guest@portfolio ")
        .title_bottom(format!(" {} ", app.last_status));
    let inner = shell.inner(area);
    frame.render_widget(shell, area);

    if app.is_mobile_layout(area) {
        render_mobile_shell(frame, inner, app, palette);
    } else {
        render_desktop_shell(frame, inner, app, palette);
    }
}

fn render_desktop_shell(frame: &mut Frame<'_>, area: Rect, app: &mut App, palette: Palette) {
    let header_height = if area.height >= 28 && area.width >= 64 {
        8
    } else if area.height >= 12 {
        3
    } else {
        1
    };
    let history_height = if area.height >= 25 {
        6
    } else if area.height >= 18 {
        4
    } else {
        3
    };

    let [header, body, history, prompt] = Layout::vertical([
        Constraint::Length(header_height),
        Constraint::Min(5),
        Constraint::Length(history_height),
        Constraint::Length(3),
    ])
    .areas(area);

    render_header(frame, header, app, palette);
    panels::render_active_panel(frame, body, app, palette, false);
    render_history(frame, history, app, palette);
    render_prompt(frame, prompt, app, palette, false);
}

fn render_mobile_shell(frame: &mut Frame<'_>, area: Rect, app: &mut App, palette: Palette) {
    // Phone screens prioritize the active panel. The shortcuts and prompt stay
    // available, but command history is desktop-only so it cannot crowd out
    // the content someone came to read or tap.
    let [header, body, chips, prompt] = mobile_shell_areas(area);

    render_mobile_header(frame, header, app, palette);
    panels::render_active_panel(frame, body, app, palette, true);
    render_command_chips(frame, chips, app, palette);
    render_prompt(frame, prompt, app, palette, true);
}

fn mobile_shell_areas(area: Rect) -> [Rect; 4] {
    let chips_height = if area.width >= 34 { 4 } else { 5 };
    Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(4),
        Constraint::Length(chips_height),
        Constraint::Length(3),
    ])
    .areas(area)
}

fn render_header(frame: &mut Frame<'_>, area: Rect, app: &App, palette: Palette) {
    if area.is_empty() {
        return;
    }

    let lines = if area.height >= 7 && area.width >= 64 {
        let mut lines = BANNER
            .iter()
            .map(|line| {
                Line::from(Span::styled(
                    *line,
                    Style::default()
                        .fg(palette.accent)
                        .add_modifier(Modifier::BOLD),
                ))
                .alignment(Alignment::Center)
            })
            .collect::<Vec<_>>();
        lines.push(Line::from("Rust/WASM portfolio shell").alignment(Alignment::Center));
        lines.push(status_line(app, palette).alignment(Alignment::Center));
        lines
    } else if area.height > 1 {
        vec![
            Line::from(vec![
                Span::styled(
                    "Kevius Tribble",
                    Style::default()
                        .fg(palette.accent)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(" | Rust/WASM portfolio shell"),
            ]),
            status_line(app, palette),
        ]
    } else {
        vec![Line::from(vec![
            Span::styled("Kevius Tribble", Style::default().fg(palette.accent)),
            Span::raw(" | "),
            Span::raw(app.active_panel.title()),
            Span::raw(" | "),
            Span::raw(app.theme.name()),
        ])]
    };

    frame.render_widget(
        Paragraph::new(Text::from(lines))
            .style(Style::default().fg(palette.foreground))
            .wrap(Wrap { trim: false }),
        area,
    );
}

fn render_mobile_header(frame: &mut Frame<'_>, area: Rect, app: &App, palette: Palette) {
    if area.is_empty() {
        return;
    }

    let lines = vec![Line::from(vec![
        Span::styled(
            "Kevius Tribble",
            Style::default()
                .fg(palette.accent)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" | "),
        Span::raw(app.active_panel.title()),
    ])];

    frame.render_widget(
        Paragraph::new(Text::from(lines))
            .style(Style::default().fg(palette.foreground))
            .wrap(Wrap { trim: false }),
        area,
    );
}

fn status_line(app: &App, palette: Palette) -> Line<'static> {
    Line::from(vec![
        Span::styled("panel: ", Style::default().fg(palette.muted)),
        Span::raw(app.active_panel.title()),
        Span::raw(" | "),
        Span::styled("theme: ", Style::default().fg(palette.muted)),
        Span::raw(app.theme.name()),
        Span::raw(" | "),
        Span::styled("hint: ", Style::default().fg(palette.muted)),
        Span::raw("/help"),
    ])
}

fn render_command_chips(frame: &mut Frame<'_>, area: Rect, app: &mut App, palette: Palette) {
    if area.is_empty() {
        return;
    }

    let block = Block::bordered()
        .border_style(Style::default().fg(palette.border))
        .title(" shortcuts ");
    let inner = block.inner(area);
    frame.render_widget(block, area);

    if inner.is_empty() {
        return;
    }

    let x_limit = inner.x.saturating_add(inner.width);
    let y_limit = inner.y.saturating_add(inner.height);
    let row_height = if inner.width >= 62 && inner.height >= 2 {
        2
    } else {
        1
    };
    let mut x = inner.x;
    let mut y = inner.y;

    for &command in MOBILE_COMMANDS {
        let chip_width = (command.len() as u16).saturating_add(2).min(inner.width);
        if x > inner.x && x.saturating_add(chip_width) > x_limit {
            x = inner.x;
            y = y.saturating_add(row_height);
        }
        if y >= y_limit || chip_width == 0 {
            break;
        }

        let visible_width = chip_width.min(x_limit.saturating_sub(x));
        if visible_width == 0 {
            break;
        }

        let is_active = command_matches_panel(command, app.active_panel);
        let style = if is_active {
            Style::default()
                .fg(palette.selection_foreground)
                .bg(palette.selection_background)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(palette.accent)
        };
        let text = format!("[{command}]");
        let chip_area = Rect::new(x, y, visible_width, 1);
        frame.render_widget(Paragraph::new(text).style(style), chip_area);

        let hit_height = row_height.min(y_limit.saturating_sub(y));
        app.add_command_hit_zone(command, Rect::new(x, y, visible_width, hit_height));
        x = x.saturating_add(chip_width.saturating_add(1));
    }
}

fn command_matches_panel(command: &str, panel: Panel) -> bool {
    matches!(
        (command, panel),
        ("/projects", Panel::Projects)
            | ("/resume", Panel::Resume)
            | ("/socials", Panel::Socials)
            | ("/about", Panel::About)
            | ("/help", Panel::Help)
    )
}

fn render_history(frame: &mut Frame<'_>, area: Rect, app: &App, palette: Palette) {
    if area.is_empty() {
        return;
    }

    let mut lines = Vec::new();
    for entry in app.output_history.iter().rev().take(area.height as usize) {
        lines.push(Line::from(vec![
            Span::styled("$ ", Style::default().fg(palette.accent)),
            Span::styled(
                entry.command.as_str(),
                Style::default().fg(palette.foreground),
            ),
        ]));
        for line in &entry.lines {
            lines.push(Line::from(vec![
                Span::styled("  ", Style::default().fg(palette.muted)),
                Span::styled(line.as_str(), Style::default().fg(palette.muted)),
            ]));
        }
    }

    lines.truncate(area.height as usize);
    if lines.is_empty() {
        lines.push(Line::from(Span::styled(
            "No output. Type /help.",
            Style::default().fg(palette.muted),
        )));
    }

    frame.render_widget(
        Paragraph::new(Text::from(lines))
            .block(
                Block::bordered()
                    .border_style(Style::default().fg(palette.border))
                    .title(" output "),
            )
            .style(Style::default().fg(palette.foreground))
            .wrap(Wrap { trim: false }),
        area,
    );
}

fn render_prompt(frame: &mut Frame<'_>, area: Rect, app: &App, palette: Palette, is_mobile: bool) {
    if area.is_empty() {
        return;
    }

    let prompt = format!("guest@portfolio:~$ {}_", app.command_input);
    let title = if is_mobile {
        " command optional "
    } else {
        " command "
    };
    frame.render_widget(
        Paragraph::new(prompt)
            .block(
                Block::bordered()
                    .border_style(Style::default().fg(palette.border))
                    .title(title),
            )
            .style(
                Style::default()
                    .fg(palette.foreground)
                    .bg(palette.background_panel),
            )
            .wrap(Wrap { trim: false }),
        area,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mobile_shell_reserves_rows_for_content_not_history() {
        let [header, body, chips, prompt] = mobile_shell_areas(Rect::new(0, 0, 40, 40));

        assert_eq!(header.height, 1);
        assert_eq!(chips.height, 4);
        assert_eq!(prompt.height, 3);
        assert_eq!(body.height, 32);
    }

    #[test]
    fn narrow_mobile_shell_allows_an_extra_shortcut_row() {
        let [_header, body, chips, _prompt] = mobile_shell_areas(Rect::new(0, 0, 33, 40));

        assert_eq!(chips.height, 5);
        assert_eq!(body.height, 31);
    }
}
