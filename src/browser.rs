use std::{cell::RefCell, rc::Rc};

use ratzilla::event::{KeyCode, KeyEvent};
use ratzilla::utils::open_url;
use web_sys::{
    wasm_bindgen::{prelude::Closure, JsCast},
    Element, EventTarget, HtmlElement, KeyboardEvent, Node,
};

use crate::app::App;
use crate::theme::Theme;

pub const RESUME_PATH: &str = "/resume.pdf";
const TERMINAL_GRID_ID: &str = "terminal_ratzilla_grid";

//opend url in browser
pub fn open_external(url: &str) -> Result<(), String> {
    open_url(url, true).map_err(|error| format!("{error:?}"))
}

//open my resume
pub fn open_resume() -> Result<(), String> {
    open_external(RESUME_PATH)
}

pub fn install_key_handler(app: Rc<RefCell<App>>) -> Result<(), String> {
    let window = web_sys::window().ok_or("window unavailable")?;
    let document = window.document().ok_or("document unavailable")?;
    let event_target: EventTarget = document.clone().into();

    let closure = Closure::<dyn FnMut(KeyboardEvent)>::new(move |event: KeyboardEvent| {
        let Some(document) = web_sys::window().and_then(|window| window.document()) else {
            return;
        };

        let key = KeyEvent::from(event.clone());
        let terminal_has_focus = terminal_has_focus(&document);
        if !should_handle_key(&event, &key, terminal_has_focus) {
            return;
        }

        event.prevent_default();
        app.borrow_mut().handle_key_event(key);
        focus_terminal_grid(&document);
    });

    event_target
        .add_event_listener_with_callback_and_bool(
            "keydown",
            closure.as_ref().unchecked_ref(),
            true,
        )
        .map_err(|error| format!("{error:?}"))?;
    closure.forget();

    Ok(())
}

fn should_handle_key(event: &KeyboardEvent, key: &KeyEvent, terminal_has_focus: bool) -> bool {
    if event.alt_key() || event.meta_key() || is_editable_target(event) {
        return false;
    }

    match key.code {
        KeyCode::Char(ch) if key.ctrl => terminal_has_focus && matches!(ch, 'k' | 'K' | 'l' | 'L'),
        KeyCode::Char(_) => !key.ctrl,
        KeyCode::Backspace | KeyCode::Enter | KeyCode::Up | KeyCode::Down | KeyCode::Esc => true,
        _ => false,
    }
}

fn is_editable_target(event: &KeyboardEvent) -> bool {
    let Some(target) = event.target() else {
        return false;
    };
    let Ok(element) = target.dyn_into::<Element>() else {
        return false;
    };

    element
        .closest("input, textarea, select, [contenteditable]")
        .ok()
        .flatten()
        .is_some()
}

fn terminal_has_focus(document: &web_sys::Document) -> bool {
    let Some(terminal) = document.get_element_by_id(TERMINAL_GRID_ID) else {
        return false;
    };
    let Some(active_element) = document.active_element() else {
        return false;
    };

    let terminal_node: &Node = terminal.as_ref();
    let active_node: &Node = active_element.as_ref();
    terminal_node.contains(Some(active_node))
}

fn focus_terminal_grid(document: &web_sys::Document) {
    let Some(element) = document.get_element_by_id(TERMINAL_GRID_ID) else {
        return;
    };
    let Some(html_element) = element.dyn_ref::<HtmlElement>() else {
        return;
    };

    let _ = html_element.focus();
}

// apply theme to site
pub fn apply_theme(theme: Theme) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let Some(document) = window.document() else {
        return;
    };
    let Some(root) = document.document_element() else {
        return;
    };

    let _ = root.set_attribute("data-theme", theme.name());
}
