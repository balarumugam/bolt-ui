use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use crate::rsx;
use crate::rsx_internal;
use crate::state::actions::Action;

pub fn toggle_visibility() -> Element {
    rsx!(button {
        class = "btn btn-primary",
        "Toggle Visibility",
        click => crate::action_handler(Action::ToggleVisibility)
    })
}