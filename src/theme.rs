use wasm_bindgen::prelude::*;
use web_sys::{Element, HtmlElement};

#[derive(Copy, Clone, Default)]
pub enum Theme {
    #[default]
    Light,
    Dark,
}

impl Theme {
    pub fn to_str(&self) -> &'static str {
        match self {
            Theme::Light => "theme-light",
            Theme::Dark => "theme-dark",
        }
    }
}

impl std::fmt::Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

#[derive(Clone)]
pub struct ThemeProvider {
    current_theme: Theme,
}

impl ThemeProvider {
    pub fn new() -> Self {
        Self {
            current_theme: Theme::Light,
        }
    }

    pub fn toggle_theme(&mut self) {
        self.current_theme = match self.current_theme {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        };
    }

    pub fn apply_theme_to_element(&self, element: &Element) -> Result<(), JsValue> {
        let html_element = element.dyn_ref::<HtmlElement>()
            .ok_or_else(|| JsValue::from_str("Element is not an HtmlElement"))?;
        
        // Add theme class
        html_element.set_class_name(&format!("app-container {}", self.current_theme.to_str()));
        
        Ok(())
    }
}

impl Default for ThemeProvider {
    fn default() -> Self {
        Self::new()
    }
}