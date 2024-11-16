use crate::theme::{Theme, ThemeProvider};
use crate::router::Route;  // Update this import

#[derive(Clone, Default)]
pub struct Todo {
    pub text: String,
    pub completed: bool,
}

#[derive(Copy, Clone, Default)]
pub enum Visibility {
    #[default]
    Shown,
    Hidden,
}

#[derive(Clone)]
pub struct AppState {
    pub counter: i32,
    pub theme: Theme,
    pub visibility: Visibility,
    pub todos: Vec<Todo>,
    pub theme_provider: ThemeProvider,
    pub current_route: Route,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            counter: 0,
            theme: Theme::default(),
            visibility: Visibility::default(),
            todos: Vec::new(),
            theme_provider: ThemeProvider::new(),
            current_route: Route::Home,
        }
    }
}