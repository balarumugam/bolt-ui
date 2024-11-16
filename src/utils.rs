use wasm_bindgen::prelude::*;
use web_sys::{window, console, Document};

pub fn get_document() -> Document {
    window()
        .expect("no global `window` exists")
        .document()
        .expect("should have a document on window")
}

// pub fn merge_styles<'a>(base: &'a str, custom: Option<&'a str>) -> &'a str {
//     match custom {
//         Some(custom_styles) => &format!("{} {}", base, custom_styles),
//         None => base,
//     }
// }

// pub fn console_log(s: &str) {
//     web_sys::console::log_1(&JsValue::from_str(s));
// }