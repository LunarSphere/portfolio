use ratzilla::ratatui::{
    prelude::*,
    widgets::{Block, BorderType, Paragraph, Wrap},
};

use crate::{
    app::{App, Panel},
    browser,
    command::COMMANDS,
    data::{Project, SocialLink},
    theme::Palette,
};

use super::list::{SelectableItem, SelectableList};

pub fn render_active_panel(
    frame: &mut Frame<'_>,
    area: Rect,
    app: &mut App,
    palette: Palette,
    is_mobile: bool,
) {
    match app.active_panel {
        Panel::Welcome => render_welcome(frame, area, app, palette, is_mobile),
        Panel::Help => render_help(frame, area, palette, is_mobile),
        Panel::About => render_about(frame, area, palette),
        Panel::Projects => render_projects(frame, area, app, palette, is_mobile),
        Panel::Socials => render_socials(frame, area, app, palette, is_mobile),
        Panel::Resume => render_resume(frame, area, palette),
    }
}

fn render_welcome(frame: &mut Frame<'_>, area: Rect, app: &App, palette: Palette, is_mobile: bool) {
    let project_count = app.projects.len();
    let social_count = app.socials.len();
    let mut lines = vec![
        Line::from(Span::styled(
            "James Kevius Tribble",
            Style::default()
                .fg(palette.accent)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from("Rust/WASM portfolio rendered as a browser TUI."),
        Line::from(""),
    ];
    if !is_mobile {
        lines.push(Line::from(vec![
            Span::styled("Fast path: ", Style::default().fg(palette.muted)),
            Span::raw("/projects"),
            Span::raw("  "),
            Span::raw("/resume"),
            Span::raw("  "),
            Span::raw("/socials"),
            Span::raw("  "),
            Span::raw("/help"),
        ]));
    }
    lines.extend([
        Line::from(vec![
            Span::styled("Loaded: ", Style::default().fg(palette.muted)),
            Span::raw(format!("{project_count} projects, {social_count} socials")),
        ]),
        Line::from(""),
        Line::from(if is_mobile {
            "Tap a shortcut below or type a command."
        } else {
            "Type a command in the prompt and press Enter."
        }),
    ]);

    render_text_panel(frame, area, " welcome ", lines, palette);
}

fn render_help(frame: &mut Frame<'_>, area: Rect, palette: Palette, is_mobile: bool) {
    let mut lines = vec![
        Line::from(Span::styled(
            "Commands",
            Style::default()
                .fg(palette.accent)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
    ];

    for command in COMMANDS {
        lines.push(Line::from(vec![
            Span::styled(
                format!("{:<10}", command.name),
                Style::default()
                    .fg(palette.foreground)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(command.description, Style::default().fg(palette.muted)),
        ]));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled("Lists: ", Style::default().fg(palette.muted)),
        Span::raw(if is_mobile {
            "tap an item to open its primary link."
        } else {
            "Up/Down or hover selects; Enter opens the selected primary URL."
        }),
    ]));
    lines.push(Line::from(vec![
        Span::styled("Escape: ", Style::default().fg(palette.muted)),
        Span::raw("clear input or return to welcome."),
    ]));

    render_text_panel(frame, area, " help ", lines, palette);
}

//about I'll need to edit this
fn render_about(frame: &mut Frame<'_>, area: Rect, palette: Palette) {
    let lines = vec![
        Line::from(Span::styled(
            "About",
            Style::default()
                .fg(palette.accent)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("Kevius Tribble is a Computer Science Student @ Clemson University."),
        Line::from("Passionate about solving Computer Vision, Cloud, and ML Problems."),
        Line::from(""),
        Line::from(vec![
            Span::styled("Next: ", Style::default().fg(palette.muted)),
            Span::raw("/projects for work samples, /resume for the PDF, /socials for links."),
        ]),
    ];

    render_text_panel(frame, area, " about ", lines, palette);
}

fn render_resume(frame: &mut Frame<'_>, area: Rect, palette: Palette) {
    let lines = vec![
        Line::from(Span::styled(
            "Resume",
            Style::default()
                .fg(palette.accent)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("The browser has been asked to open the resume in a new tab."),
        Line::from("Browsers control whether that opens inline, downloads, or prompts."),
        Line::from(""),
        Line::from(vec![
            Span::styled("Direct path: ", Style::default().fg(palette.muted)),
            Span::styled(
                browser::RESUME_PATH,
                Style::default().fg(palette.foreground),
            ),
        ]),
        Line::from("Replace public/resume.pdf with the current resume before publishing."),
    ];

    render_text_panel(frame, area, " resume ", lines, palette);
}

fn render_projects(
    frame: &mut Frame<'_>,
    area: Rect,
    app: &mut App,
    palette: Palette,
    is_mobile: bool,
) {
    let rows = app
        .projects
        .iter()
        .map(|project| SelectableItem {
            title: project.title.clone(),
            meta: project.technologies.join(", "),
            marker: project.featured.then_some("*"),
        })
        .collect();

    if is_mobile {
        SelectableList::new(" projects ", Panel::Projects, rows)
            .render(frame, area, app, palette, true);
        return;
    }

    let [list_area, preview_area] = split_browser_area(area, is_mobile);
    SelectableList::new(" projects ", Panel::Projects, rows)
        .render(frame, list_area, app, palette, is_mobile);
    render_project_preview(frame, preview_area, app.selected_project(), palette);
}

fn render_socials(
    frame: &mut Frame<'_>,
    area: Rect,
    app: &mut App,
    palette: Palette,
    is_mobile: bool,
) {
    let rows = app
        .socials
        .iter()
        .map(|social| SelectableItem {
            title: social.label.clone(),
            meta: social.handle.clone().unwrap_or_default(),
            marker: None,
        })
        .collect();

    if is_mobile {
        SelectableList::new(" socials ", Panel::Socials, rows)
            .render(frame, area, app, palette, true);
        return;
    }

    let [list_area, preview_area] = split_browser_area(area, is_mobile);
    SelectableList::new(" socials ", Panel::Socials, rows)
        .render(frame, list_area, app, palette, is_mobile);
    render_social_preview(frame, preview_area, app.selected_social(), palette);
}

fn render_project_preview(
    frame: &mut Frame<'_>,
    area: Rect,
    project: Option<&Project>,
    palette: Palette,
) {
    let Some(project) = project else {
        render_text_panel(
            frame,
            area,
            " preview ",
            vec![Line::from("No project selected.")],
            palette,
        );
        return;
    };

    let mut lines = vec![
        Line::from(Span::styled(
            project.title.as_str(),
            Style::default()
                .fg(palette.accent)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            project.blurb.as_str(),
            Style::default().fg(palette.foreground),
        )),
        Line::from(""),
        Line::from(project.description.as_str()),
        Line::from(""),
        Line::from(vec![
            Span::styled("Tech: ", Style::default().fg(palette.muted)),
            Span::raw(project.technologies.join(", ")),
        ]),
    ];

    push_optional_url(&mut lines, "Live", project.live_url.as_deref(), palette);
    push_optional_url(&mut lines, "GitHub", project.github_url.as_deref(), palette);

    if project.primary_url().is_none() {
        lines.push(Line::from(Span::styled(
            "No URL configured yet.",
            Style::default().fg(palette.warning),
        )));
    }

    render_text_panel(frame, area, " preview ", lines, palette);
}

fn render_social_preview(
    frame: &mut Frame<'_>,
    area: Rect,
    social: Option<&SocialLink>,
    palette: Palette,
) {
    let Some(social) = social else {
        render_text_panel(
            frame,
            area,
            " preview ",
            vec![Line::from("No social selected.")],
            palette,
        );
        return;
    };

    let mut lines = vec![
        Line::from(Span::styled(
            social.label.as_str(),
            Style::default()
                .fg(palette.accent)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            social.handle.as_deref().unwrap_or(""),
            Style::default().fg(palette.muted),
        )),
        Line::from(""),
        Line::from(social.blurb.as_str()),
        Line::from(""),
    ];
    push_optional_url(&mut lines, "URL", social.primary_url(), palette);

    render_text_panel(frame, area, " preview ", lines, palette);
}

fn push_optional_url<'a>(
    lines: &mut Vec<Line<'a>>,
    label: &'static str,
    url: Option<&'a str>,
    palette: Palette,
) {
    if let Some(url) = url.filter(|url| !url.trim().is_empty()) {
        lines.push(Line::from(vec![
            Span::styled(format!("{label}: "), Style::default().fg(palette.muted)),
            Span::styled(url.to_string(), Style::default().fg(palette.success)),
        ]));
    }
}

fn split_browser_area(area: Rect, is_mobile: bool) -> [Rect; 2] {
    if is_mobile {
        Layout::vertical([Constraint::Percentage(48), Constraint::Percentage(52)]).areas(area)
    } else if area.width >= 82 && area.height >= 12 {
        Layout::horizontal([Constraint::Percentage(38), Constraint::Percentage(62)]).areas(area)
    } else {
        Layout::vertical([Constraint::Percentage(45), Constraint::Percentage(55)]).areas(area)
    }
}

fn render_text_panel<'a>(
    frame: &mut Frame<'_>,
    area: Rect,
    title: &'static str,
    lines: Vec<Line<'a>>,
    palette: Palette,
) {
    let block = Block::bordered()
        .border_type(BorderType::Plain)
        .border_style(Style::default().fg(palette.border))
        .title(title);

    frame.render_widget(
        Paragraph::new(Text::from(lines))
            .block(block)
            .style(
                Style::default()
                    .bg(palette.background)
                    .fg(palette.foreground),
            )
            .wrap(Wrap { trim: false }),
        area,
    );
}
