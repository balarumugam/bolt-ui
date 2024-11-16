use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use crate::rsx;
use crate::rsx_internal;
use crate::state::actions::Action;

pub fn toggle_theme_button() -> Element {
    rsx!(button {
        class = "btn btn-primary",
        "Toggle Theme",
        click => crate::action_handler(Action::ToggleTheme)
    })
}