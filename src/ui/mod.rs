use ratzilla::ratatui::{
    prelude::*,
    widgets::{Block, BorderType, Clear},
};

use crate::app::App;

mod background;
mod list;
mod panels;
mod shell;

pub use background::LifeBackground;

pub fn render(frame: &mut Frame<'_>, app: &mut App, background: &mut LifeBackground) {
    app.clear_hit_zones();
    let palette = app.theme.palette();
    let area = frame.area();

    frame.render_widget(
        Block::default().style(Style::default().bg(palette.background)),
        area,
    );
    let is_mobile = app.is_mobile_layout(area);
    background.render(frame, palette, is_mobile);

    let shell_area = background_area(area, is_mobile);
    render_background(frame, shell_area, palette);
    shell::render(frame, shell_area, app, palette);
}

fn background_area(area: Rect, is_mobile: bool) -> Rect {
    if is_mobile || area.width < 72 || area.height < 24 {
        return area;
    }

    let width = area.width.saturating_mul(88) / 100;
    let height = area.height.saturating_mul(86) / 100;
    Rect::new(
        area.x + area.width.saturating_sub(width) / 2,
        area.y + area.height.saturating_sub(height) / 2,
        width,
        height,
    )
}

fn render_background(frame: &mut Frame<'_>, area: Rect, palette: crate::theme::Palette) {
    let mut area = Rect::new(
        area.x.saturating_sub(2),
        area.y.saturating_sub(1),
        area.width.saturating_add(4),
        area.height.saturating_add(2),
    );
    area = area.clamp(frame.area());

    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(palette.accent))
        .style(
            Style::default()
                .fg(palette.accent)
                .bg(palette.background_panel),
        )
        .title_bottom(" | built with Ratzilla | ")
        .title_alignment(Alignment::Right);

    frame.render_widget(Clear, area);
    frame.render_widget(block, area);
}
