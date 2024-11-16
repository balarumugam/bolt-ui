use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{Element, console, HtmlElement, HtmlInputElement};
use crate::rsx;
use crate::rsx_internal;
use crate::state::{STATE, actions::{Action, TodoOperation}};
use crate::utils::get_document;

// Add a capacity hint for better Vec performance
pub fn add_todo(text: String) {
    crate::state::update_state(|mut state| {
        // Reserve space if needed
        if state.todos.capacity() == state.todos.len() {
            state.todos.reserve(32); // Reserve in chunks to avoid frequent reallocations
        }
        state.todos.push(crate::state::app_state::Todo {
            text,
            completed: false,
        });
        state
    });
}

pub fn handle_add_todo(event: Option<web_sys::Event>) {
    if let Some(input) = get_document()
        .get_element_by_id("todo-input")
        .and_then(|el| el.dyn_into::<HtmlInputElement>().ok()) 
    {
        let text = input.value();
        if !text.is_empty() {
            add_todo(text);
            input.set_value("");
        }
    }
}

pub fn todo_input() -> Element {
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

fn render_todo_item(index: usize, todo: &crate::state::app_state::Todo) -> Element {
    let toggle_action = Action::Todo(TodoOperation::Toggle(index));
    let remove_action = Action::Todo(TodoOperation::Remove(index));
    
    rsx!(div {
        class = if todo.completed { "todo-item completed" } else { "todo-item" },
        input {
            class = "todo-checkbox",
            type = "checkbox",
            checked = todo.completed,
            click => crate::action_handler(toggle_action)
        },
        span { 
            class = "todo-text",
            @&todo.text
        },
        button {
            class = "todo-delete",
            "Delete",
            click => crate::action_handler(remove_action)
        }
    })
}

pub fn todo_list() -> Element {
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

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    use web_sys::{window, HtmlElement};
    use crate::state::app_state::Todo;

    wasm_bindgen_test_configure!(run_in_browser);

    fn setup_todo_input() -> HtmlInputElement {
        let doc = get_document();
        let input = doc.create_element("input")
            .expect("should create input")
            .dyn_into::<HtmlInputElement>()
            .expect("should be input element");
        
        input.set_id("todo-input");
        doc.body()
            .expect("should have body")
            .append_child(&input)
            .expect("should append input");
        
        input
    }

    fn cleanup_todo_input() {
        if let Some(input) = get_document().get_element_by_id("todo-input") {
            input.parent_element()
                .expect("should have parent")
                .remove_child(&input)
                .expect("should remove input");
        }
    }

    #[wasm_bindgen_test]
    fn test_add_todo() {
        // Setup
        STATE.with(|state| {
            let mut state = state.borrow_mut();
            state.todos.clear();
        });

        // Test adding a todo
        add_todo("Test todo".to_string());

        STATE.with(|state| {
            let state = state.borrow();
            assert_eq!(state.todos.len(), 1);
            assert_eq!(state.todos[0].text, "Test todo");
            assert_eq!(state.todos[0].completed, false);
        });
    }

    #[wasm_bindgen_test]
    fn test_handle_add_todo() {
        // Setup
        let input = setup_todo_input();
        STATE.with(|state| {
            let mut state = state.borrow_mut();
            state.todos.clear();
        });

        // Test adding todo via handler
        input.set_value("Test todo via handler");
        handle_add_todo(None);

        STATE.with(|state| {
            let state = state.borrow();
            assert_eq!(state.todos.len(), 1);
            assert_eq!(state.todos[0].text, "Test todo via handler");
            assert_eq!(input.value(), ""); // Input should be cleared
        });

        // Cleanup
        cleanup_todo_input();
    }

    #[wasm_bindgen_test]
    fn test_todo_rendering() {
        // Setup
        STATE.with(|state| {
            let mut state = state.borrow_mut();
            state.todos.clear();
            state.todos.push(Todo {
                text: "Test todo".to_string(),
                completed: false,
            });
        });

        // Test rendering
        let todo_list_element = todo_list();
        assert!(todo_list_element.query_selector(".todo-item").unwrap().is_some());
        
        // Test todo item content
        let todo_text = todo_list_element
            .query_selector(".todo-text")
            .unwrap()
            .unwrap()
            .text_content()
            .unwrap();
        assert_eq!(todo_text.trim(), "Test todo");
    }

   
    #[wasm_bindgen_test]
fn test_todo_performance() {
    let window = window().expect("should have window");
    let performance = window.performance().expect("should have performance");
    
    console::group_1(&"Todo Performance Metrics".into());

    // Setup
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.todos.clear();
    });

    let test_cases = vec![
        (10, "Small batch", 100.0),     // 100ms threshold for small batch
        (50, "Medium batch", 500.0),     // 500ms threshold for medium batch
        (100, "Large batch", 1000.0),    // 1000ms threshold for large batch
    ];

    for (iterations, test_name, threshold) in test_cases {
        // Warm-up run
        for i in 0..5 {
            add_todo(format!("Warmup Todo {}", i));
        }
        STATE.with(|state| {
            let mut state = state.borrow_mut();
            state.todos.clear();
        });

        // Actual test
        let start = performance.now();

        // Add todos
        for i in 0..iterations {
            add_todo(format!("Todo {}", i));
        }

        let duration = performance.now() - start;
        let ops_per_ms = iterations as f64 / duration;

        let message = format!(
            "{}: {:.2}ms ({:.2} ops/ms) for {} todos",
            test_name, duration, ops_per_ms, iterations
        );
        console::log_1(&message.into());

        // Verify todos were added correctly
        STATE.with(|state| {
            let state = state.borrow();
            assert_eq!(
                state.todos.len(),
                iterations,
                "Expected {} todos, got {}",
                iterations,
                state.todos.len()
            );
        });

        // Clear todos for next test
        STATE.with(|state| {
            let mut state = state.borrow_mut();
            state.todos.clear();
        });

        // Performance assertion with more realistic thresholds
        assert!(
            duration < threshold,
            "{}: Adding {} todos took {:.2}ms ({:.2} ops/ms), exceeded threshold of {}ms",
            test_name,
            iterations,
            duration,
            ops_per_ms,
            threshold
        );
    }

    console::group_end();
}
}