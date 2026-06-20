use ratzilla::ratatui::{
    prelude::*,
    widgets::{Block, BorderType, Paragraph, Wrap},
};

use crate::{app::App, theme::Palette};

use super::panels;

const BANNER: &[&str] = &[
    r#" __   ___  _______  ___      ___  __     ____  ____   ________      ___________  _______    __     _______   _______   ___       _______  "#,
    r#"|/"| /  ")/"     "||"  \    /"  ||" \   ("  _||_ " | /"       )    ("     _   ")/"      \  |" \   |   _  "\ |   _  "\ |"  |     /"     "| "#,
    r#"(: |/   /(: ______) \   \  //  / ||  |  |   (  ) : |(:   \___/      )__/  \\__/|:        | ||  |  (. |_)  :)(. |_)  :)||  |    (: ______) "#,
    r#"|    __/  \/    |    \\  \/. ./  |:  |  (:  |  | . ) \___  \           \\_ /   |_____/   ) |:  |  |:     \/ |:     \/ |:  |     \/    |   "#,
    r#"(// _  \  // ___)_    \.    //   |.  |   \\ \__/ //   __/  \\          |.  |    //      /  |.  |  (|  _  \\ (|  _  \\  \  |___  // ___)_  "#,
    r#"|: | \  \(:      "|    \\   /    /\  |\  /\\ __ //\  /" \   :)         \:  |   |:  __   \  /\  |\ |: |_)  :)|: |_)  :)( \_|:  \(:      "| "#,
    r#"(__|  \__)\_______)     \__/    (__\_|_)(__________)(_______/           \__|   |__|  \___)(__\_|_)(_______/ (_______/  \_______)\_______) "#,
    r#"                                                                                                                                          "#,
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

    let header_height = if inner.height >= 28 && inner.width >= 64 {
        8
    } else if inner.height >= 12 {
        3
    } else {
        1
    };
    let history_height = if inner.height >= 25 {
        6
    } else if inner.height >= 18 {
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
    .areas(inner);

    render_header(frame, header, app, palette);
    panels::render_active_panel(frame, body, app, palette);
    render_history(frame, history, app, palette);
    render_prompt(frame, prompt, app, palette);
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

fn render_prompt(frame: &mut Frame<'_>, area: Rect, app: &App, palette: Palette) {
    if area.is_empty() {
        return;
    }

    let prompt = format!("guest@portfolio:~$ {}_", app.command_input);
    frame.render_widget(
        Paragraph::new(prompt)
            .block(
                Block::bordered()
                    .border_style(Style::default().fg(palette.border))
                    .title(" command "),
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
