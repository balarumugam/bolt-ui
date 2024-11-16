pub mod actions;
pub mod app_state;

use core::cell::RefCell;
use crate::utils::get_document;
use app_state::AppState;
use actions::Action;

thread_local! {
    pub static STATE: RefCell<AppState> = RefCell::new(AppState::default());
}

pub fn update_state<F>(updater: F)
where
    F: FnOnce(AppState) -> AppState,
{
    STATE.with(|state| {
        let new_state = updater(state.borrow().clone());
        *state.borrow_mut() = new_state;
    });
    crate::render();
}

pub fn dispatch(action: Action) {
    update_state(|mut state| {
        match action {
            Action::Counter(op) => {
                state.counter = match op {
                    actions::Operation::Increment => state.counter + 1,
                    actions::Operation::Decrement => state.counter - 1,
                    actions::Operation::Reset => 0,
                };
            },
            Action::Todo(op) => {
                match op {
                    actions::TodoOperation::Toggle(index) => {
                        if let Some(todo) = state.todos.get_mut(index) {
                            todo.completed = !todo.completed;
                        }
                    },
                    actions::TodoOperation::Remove(index) => {
                        if index < state.todos.len() {
                            state.todos.remove(index);
                        }
                    },
                }
            },
            Action::ToggleTheme => {
                state.theme = match state.theme {
                    crate::theme::Theme::Light => crate::theme::Theme::Dark,
                    crate::theme::Theme::Dark => crate::theme::Theme::Light,
                };
                state.theme_provider.toggle_theme();
                
                if let Some(root) = get_document().get_element_by_id("app") {
                    state.theme_provider.apply_theme_to_element(&root)
                        .expect("Failed to apply theme");
                }
            },
            Action::ToggleVisibility => {
                state.visibility = match state.visibility {
                    app_state::Visibility::Shown => app_state::Visibility::Hidden,
                    app_state::Visibility::Hidden => app_state::Visibility::Shown,
                };
            },
        }
        state
    });
}