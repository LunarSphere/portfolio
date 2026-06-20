use ratzilla::ratatui::{prelude::*, widgets::Paragraph};

use crate::theme::Palette;

const UPDATE_EVERY_FRAMES: u8 = 5;

#[derive(Debug, Default)]
pub struct LifeBackground {
    width: u16,
    height: u16,
    cells: Vec<bool>,
    scratch: Vec<bool>,
    frame_count: u8,
}

impl LifeBackground {
    pub fn render(&mut self, frame: &mut Frame<'_>, palette: Palette) {
        let area = frame.area();
        if area.is_empty() {
            return;
        }

        self.ensure_size(area.width, area.height);
        self.frame_count = self.frame_count.saturating_add(1);
        if self.frame_count >= UPDATE_EVERY_FRAMES {
            self.frame_count = 0;
            self.step();
        }

        let lines = self
            .cells
            .chunks(self.width as usize)
            .map(|row| {
                let line = row
                    .iter()
                    .map(|alive| if *alive { '.' } else { ' ' })
                    .collect::<String>();
                Line::from(line)
            })
            .collect::<Vec<_>>();

        frame.render_widget(
            Paragraph::new(Text::from(lines)).style(Style::default().fg(palette.background_life)),
            area,
        );
    }

    fn ensure_size(&mut self, width: u16, height: u16) {
        if width == 0 || height == 0 || (self.width == width && self.height == height) {
            return;
        }

        self.width = width;
        self.height = height;
        let len = width as usize * height as usize;
        self.cells = (0..len)
            .map(|index| seeded_cell(index as u64, width, height))
            .collect();
        self.scratch = vec![false; len];
        self.frame_count = 0;
    }

    fn step(&mut self) {
        if self.width == 0 || self.height == 0 {
            return;
        }

        for y in 0..self.height {
            for x in 0..self.width {
                let index = self.index(x, y);
                let neighbors = self.live_neighbors(x, y);
                self.scratch[index] =
                    matches!((self.cells[index], neighbors), (true, 2 | 3) | (false, 3));
            }
        }

        std::mem::swap(&mut self.cells, &mut self.scratch);
    }

    fn live_neighbors(&self, x: u16, y: u16) -> u8 {
        let mut count = 0;
        for y_offset in [-1, 0, 1] {
            for x_offset in [-1, 0, 1] {
                if x_offset == 0 && y_offset == 0 {
                    continue;
                }

                let Some(nx) = offset_coordinate(x, x_offset, self.width) else {
                    continue;
                };
                let Some(ny) = offset_coordinate(y, y_offset, self.height) else {
                    continue;
                };
                count += u8::from(self.cells[self.index(nx, ny)]);
            }
        }
        count
    }

    fn index(&self, x: u16, y: u16) -> usize {
        y as usize * self.width as usize + x as usize
    }
}

fn offset_coordinate(value: u16, offset: i16, limit: u16) -> Option<u16> {
    value
        .checked_add_signed(offset)
        .filter(|value| *value < limit)
}

fn seeded_cell(index: u64, width: u16, height: u16) -> bool {
    let mut value = index
        .wrapping_mul(0x9e37_79b9_7f4a_7c15)
        .wrapping_add((width as u64) << 32)
        .wrapping_add(height as u64);
    value ^= value >> 30;
    value = value.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    value ^= value >> 27;
    value = value.wrapping_mul(0x94d0_49bb_1331_11eb);
    value ^= value >> 31;
    value % 5 == 0
}
