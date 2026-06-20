use std::{cell::RefCell, io, rc::Rc};

use ratzilla::{ratatui::Terminal, utils::set_document_title, DomBackend, WebRenderer};

mod app;
mod browser;
mod command;
mod data;
mod theme;
mod ui;

use app::App;

fn main() -> io::Result<()> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    let _ = set_document_title("James Kevius Tribble | Portfolio");

    let app = Rc::new(RefCell::new(App::new()));
    browser::apply_theme(app.borrow().theme);
    browser::install_key_handler(Rc::clone(&app)).map_err(io::Error::other)?;

    let backend = DomBackend::new_by_id("terminal")?;
    let mut terminal = Terminal::new(backend)?;
    let mut background = ui::LifeBackground::default();

    terminal.on_mouse_event({
        let app = Rc::clone(&app);
        move |mouse| {
            app.borrow_mut().handle_mouse_event(mouse);
        }
    })?;

    terminal.draw_web(move |frame| {
        ui::render(frame, &mut app.borrow_mut(), &mut background);
    });

    Ok(())
}
