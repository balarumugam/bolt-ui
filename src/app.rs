use wasm_bindgen::prelude::*;
use web_sys::{window, Document, HtmlElement, Event, HtmlInputElement, Element};
mod theme;  // Add this if not already present
use theme::ThemeProvider; 
use core::cell::RefCell;
use std::fmt;
mod rsx;

fn get_document() -> Document {
    window().expect("not found").document().expect("not found")
}

macro_rules! append_child {
    ($parent:expr, $child:expr) => {
        $parent.append_child(&$child).expect("Failed to append child")
    };
}

// State Management
thread_local! {
    static STATE: RefCell<AppState> = RefCell::new(AppState::default());
}

#[derive(Clone)]
struct AppState {
    counter: i32,
    theme: Theme,
    visibility: Visibility,
    todos: Vec<Todo>,
    theme_provider: ThemeProvider,
}

#[derive(Clone, Default)]
struct Todo {
    text: String,
    completed: bool,
}

#[derive(Copy, Clone)]
enum Operation {
    Increment,
    Decrement,
    Reset,
}

#[derive(Copy, Clone, Default)]
enum Theme {
    #[default]
    Light,
    Dark,
}

#[derive(Copy, Clone, Default)]
enum Visibility {
    #[default]
    Shown,
    Hidden,
}

#[derive(Copy, Clone)]
enum TodoOperation {
    Toggle(usize),
    Remove(usize),
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            counter: 0,
            theme: Theme::Light,
            visibility: Visibility::Shown,
            todos: Vec::new(),
            theme_provider: ThemeProvider::new(),
        }
    }
}

// State Getters
impl AppState {
    fn counter(&self) -> i32 {
        self.counter
    }

    fn theme(&self) -> Theme {
        self.theme
    }

    fn visibility(&self) -> Visibility {
        self.visibility
    }

    fn todos(&self) -> &Vec<Todo> {
        &self.todos
    }
}

// Actions
#[derive(Copy, Clone)]
enum Action {
    Counter(Operation),
    Todo(TodoOperation),
    ToggleTheme,
    ToggleVisibility,
}

// State Updates
fn dispatch(action: Action) {
    update_state(|mut state| {
        match action {
            Action::Counter(op) => {
                state.counter = match op {
                    Operation::Increment => state.counter + 1,
                    Operation::Decrement => state.counter - 1,
                    Operation::Reset => 0,
                };
            },
            Action::Todo(op) => {
                match op {
                    TodoOperation::Toggle(index) => {
                        if let Some(todo) = state.todos.get_mut(index) {
                            todo.completed = !todo.completed;
                        }
                    },
                    TodoOperation::Remove(index) => {
                        if index < state.todos.len() {
                            state.todos.remove(index);
                        }
                    },
                }
            },
            Action::ToggleTheme => {
                state.theme = match state.theme {
                    Theme::Light => Theme::Dark,
                    Theme::Dark => Theme::Light,
                };
                state.theme_provider.toggle_theme();  // Toggle theme in provider
                
                // Apply theme to root element
                if let Some(root) = get_document().get_element_by_id("app") {
                    state.theme_provider.apply_theme_to_element(&root)
                        .expect("Failed to apply theme");
                }
            },
            Action::ToggleVisibility => {
                state.visibility = match state.visibility {
                    Visibility::Shown => Visibility::Hidden,
                    Visibility::Hidden => Visibility::Shown,
                };
            },
        }
        state
    });
}

fn update_state<F>(updater: F)
where
    F: FnOnce(AppState) -> AppState,
{
    STATE.with(|state| {
        let new_state = updater(state.borrow().clone());
        *state.borrow_mut() = new_state;
    });
    render();
}

// UI Components
fn counter_actions() -> Element {
    rsx!(div {
        class = "counter-actions",
        button {
            class = "btn btn-secondary",
            "+",
            click => action_handler(Action::Counter(Operation::Increment))
        },
        button {
            class = "btn btn-secondary",
            "-",
            click => action_handler(Action::Counter(Operation::Decrement))
        }
    })
}

fn theme_toggle() -> Element {
    rsx!(button {
        class = "btn btn-primary",
        "Toggle Theme",
        click => action_handler(Action::ToggleTheme)
    })
}

fn toggle_visibility() -> Element {
    rsx!(button {
        class = "btn btn-primary",
        "Toggle Visibility",
        click => action_handler(Action::ToggleVisibility)
    })
}

fn add_todo(text: String) {
    update_state(|mut state| {
        state.todos.push(Todo {
            text,
            completed: false,
        });
        state
    });
}

// Generic event handler that can handle both mouse and keyboard events
fn action_handler(action: Action) -> impl FnMut(web_sys::Event) {
    move |_| dispatch(action)
}

// fn handle_add_todo(event: Option<web_sys::Event>) {

//     // Try to get input from event target first
//     let input_value = event
//         .and_then(|e| e.target())
//         .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
//         .map(|input| input.value())
//         // If event target not available, try getting from DOM
//         .or_else(|| {
//             get_document()
//                 .get_element_by_id("todo-input")
//                 .and_then(|el| el.dyn_into::<web_sys::HtmlInputElement>().ok())
//                 .map(|input| input.value())
//         });

//     if let Some(text) = input_value {
//         if !text.is_empty() {
//             web_sys::console::log_1(&format!("Adding todo: {}", text).into());
//             add_todo(text.clone());
            
//             // Clear input after adding
//             if let Some(input) = get_document()
//                 .get_element_by_id("todo-input")
//                 .and_then(|el| el.dyn_into::<web_sys::HtmlInputElement>().ok()) 
//             {
//                 input.set_value("");
//             }
//         }
//     }
// }

fn handle_add_todo(event: Option<web_sys::Event>) {
    // Get the input element directly
    if let Some(input) = get_document()
        .get_element_by_id("todo-input")
        .and_then(|el| el.dyn_into::<web_sys::HtmlInputElement>().ok()) 
    {
        let text = input.value();
        if !text.is_empty() {
            add_todo(text);
            input.set_value("");
        }
    }
}

fn todo_input() -> Element {
    rsx!(div {
        class = "input-container",
        input {
            id = "todo-input",
            class = "input-field",
            type = "text",
            placeholder = "Add new todo",
            keydown => |e: web_sys::Event| {
                if let Ok(key_event) = e.dyn_into::<web_sys::KeyboardEvent>() {
                    if key_event.key() == "Enter" {
                        handle_add_todo(None);
                    }
                }
            }
        },
        button {
            "Add Todo",
            class = "btn",
            click => |_| handle_add_todo(None)
        }
    })
}

fn render_todo_item(index: usize, todo: &Todo) -> Element {
    let toggle_action = Action::Todo(TodoOperation::Toggle(index));
    let remove_action = Action::Todo(TodoOperation::Remove(index));
    
    rsx!(div {
        class = if todo.completed { "todo-item completed" } else { "todo-item" },
        input {
            class = "todo-checkbox",
            type = "checkbox",
            checked = todo.completed,
            click => action_handler(toggle_action)
        },
        span { 
            class = "todo-text",
            @&todo.text
        },
        button {
            class = "todo-delete",
            "Delete",
            click => action_handler(remove_action)
        }
    })
}

fn todo_list() -> Element {
    STATE.with(|state| {
        let state = state.borrow();
        rsx!(div {
            id = "todo-list",
            todo_input(),
            div {
                class = "todos",
                state.todos.iter().enumerate(), => |index, todo| render_todo_item(index, todo)
            }
        })
    })
}



// Rendering
fn render() {
    STATE.with(|state| {
        let state = state.borrow();
        
        // Update counter
        if let Some(elem) = get_document().get_element_by_id("count") {
            let mut buffer = itoa::Buffer::new();
            elem.set_text_content(Some(buffer.format(state.counter())));
        }

        // Update theme
        if let Some(root) = get_document().get_element_by_id("app") {
            root.set_class_name(match state.theme() {
                Theme::Light => "theme-light",
                Theme::Dark => "theme-dark",
            });
        }

        // Update visibility
        if let Some(content) = get_document().get_element_by_id("content") {
            let new_content = content.dyn_into::<web_sys::HtmlElement>().unwrap_throw();
            new_content.style()
                .set_property("display", match state.visibility() {
                    Visibility::Shown => "block",
                    Visibility::Hidden => "none",
                })
                .unwrap_throw();
        }

        // Update todo list
        if let Some(container) = get_document().get_element_by_id("todo-container") {
            let new_todo_list = todo_list();
            container.set_inner_html("");
            append_child!(container, new_todo_list);
        }
    });
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    main()
}

pub fn main() -> Result<(), JsValue> {
    let document = get_document();
    let body = document.body().expect("not found");

    STATE.with(|state| {
        let state = state.borrow();

        let app = rsx!(div {
            id = "app",
            class = format!("app-container {}", state.theme),
            div {
                id = "content",
                "Counter: ",
                span { 
                    id = "count",
                    class = "counter",
                    "0"
                },
                div {
                    id = "todo-container",
                    todo_list()
                }
            },
            toggle_visibility(),
            counter_actions(),
            theme_toggle()
        });

        append_child!(body, &app);

        if let Some(root) = document.get_element_by_id("app") {
            state.theme_provider.apply_theme_to_element(&root)
                .expect("Failed to apply initial theme");
        }
    });

    // append_child!(body, &input_component);
    Ok(())
}

impl fmt::Display for Theme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Theme::Light => write!(f, "theme-light"),
            Theme::Dark => write!(f, "theme-dark"),
        }
    }
}