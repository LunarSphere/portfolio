use ratzilla::ratatui::{
    prelude::*,
    widgets::{Block, BorderType, Paragraph},
};

use crate::{
    app::{App, Panel},
    theme::Palette,
};

#[derive(Debug, Clone)]
pub struct SelectableItem {
    pub title: String,
    pub meta: String,
    pub marker: Option<&'static str>,
}

pub struct SelectableList {
    title: &'static str,
    panel: Panel,
    items: Vec<SelectableItem>,
}

impl SelectableList {
    pub fn new(title: &'static str, panel: Panel, items: Vec<SelectableItem>) -> Self {
        Self {
            title,
            panel,
            items,
        }
    }

    pub fn render(self, frame: &mut Frame<'_>, area: Rect, app: &mut App, palette: Palette) {
        let block = Block::bordered()
            .border_type(BorderType::Plain)
            .border_style(Style::default().fg(palette.border))
            .title(format!(" {} ", self.title));
        let inner = block.inner(area);
        frame.render_widget(block, area);

        if inner.is_empty() {
            return;
        }

        let visible_rows = inner.height as usize;
        match self.panel {
            Panel::Projects => app.ensure_project_visible(visible_rows),
            Panel::Socials => app.ensure_social_visible(visible_rows),
            _ => {}
        }

        let selected = match self.panel {
            Panel::Projects => app.selected_project_index,
            Panel::Socials => app.selected_social_index,
            _ => 0,
        };
        let scroll = match self.panel {
            Panel::Projects => app.project_scroll,
            Panel::Socials => app.social_scroll,
            _ => 0,
        };

        for (row_offset, (index, item)) in self
            .items
            .iter()
            .enumerate()
            .skip(scroll)
            .take(visible_rows)
            .enumerate()
        {
            let y = inner.y.saturating_add(row_offset as u16);
            let row = Rect::new(inner.x, y, inner.width, 1);
            let is_selected = index == selected;
            let style = if is_selected {
                Style::default()
                    .fg(palette.selection_foreground)
                    .bg(palette.selection_background)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(palette.foreground)
            };

            let marker = item.marker.unwrap_or(" ");
            let cursor = if is_selected { ">" } else { " " };
            let line = Line::from(vec![
                Span::styled(cursor, Style::default().fg(palette.accent)),
                Span::raw(" "),
                Span::styled(marker, Style::default().fg(palette.warning)),
                Span::raw(" "),
                Span::styled(item.title.as_str(), style),
                Span::styled("  ", style),
                Span::styled(item.meta.as_str(), Style::default().fg(palette.muted)),
            ]);

            frame.render_widget(Paragraph::new(line).style(style), row);
            app.add_hit_zone(self.panel, index, row);
        }

        if self.items.is_empty() {
            frame.render_widget(
                Paragraph::new("No entries configured.").style(Style::default().fg(palette.muted)),
                inner,
            );
        }
    }
}
