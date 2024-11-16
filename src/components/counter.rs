use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use crate::rsx;
use crate::rsx_internal;
use crate::state::actions::{Action, Operation};

pub fn counter_actions() -> Element {
    rsx!(div {
        class = "counter-actions",
        button {
            class = "btn btn-secondary",
            "+",
            click => crate::action_handler(Action::Counter(Operation::Increment))
        },
        button {
            class = "btn btn-secondary",
            "-",
            click => crate::action_handler(Action::Counter(Operation::Decrement))
        }
    })
}