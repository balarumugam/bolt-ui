mod macros;
mod components;
mod state;
mod theme;
mod utils;
mod rsx;
mod content_loader;
mod router;
mod performance;

use performance::{measure, log_stats};
use router::{Route, get_current_route, navigate_to};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element,HtmlElement};
use components::*;
use state::{STATE, dispatch, app_state::AppState};
use theme::Theme;
// use components::styled_button::{styled_button};
use crate::utils::get_document;
use crate::todo::todo_list;
use crate::counter::counter_actions;
use crate::toggle_theme::toggle_theme_button;
use crate::visibility::toggle_visibility;
use content_loader::{Content, load_content};

// Generic event handler that can handle both mouse and keyboard events
pub fn action_handler(action: state::actions::Action) -> impl FnMut(web_sys::Event) {
    move |_| dispatch(action)
}

// Rendering
fn render() {
    STATE.with(|state| {
        let state = state.borrow();

        if let Some(app) = get_document().get_element_by_id("app") {
            // Update content based on route
            let new_content = render_route_content(&state.current_route);
            app.set_inner_html("");
            append_child!(app, &render_nav());
            append_child!(app, &new_content);
            append_child!(app, &toggle_visibility());
            append_child!(app, &counter_actions());
            append_child!(app, &toggle_theme_button());
        }
        
        // Update counter
        if let Some(elem) = get_document().get_element_by_id("count") {
            let mut buffer = itoa::Buffer::new();
            elem.set_text_content(Some(buffer.format(state.counter)));
        }

        // Update theme
        if let Some(root) = get_document().get_element_by_id("app") {
            root.set_class_name(&state.theme.to_str());
        }

        // Update visibility
        if let Some(content) = get_document().get_element_by_id("content") {
            let new_content = content.dyn_into::<HtmlElement>().unwrap_throw();
            new_content.style()
                .set_property("display", match state.visibility {
                    state::app_state::Visibility::Shown => "block",
                    state::app_state::Visibility::Hidden => "none",
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

fn render_articles(content: Content) -> Element {
    rsx!(
        section {
            id = "articles",
            class = "articles-section",
            content.articles.iter().enumerate(), => |i, article| rsx!(div {
                class = "article-card",
                h2 { @&article.title },
                div {
                    class = "article-meta",
                    span { @&article.date },
                    span { @&article.author }
                },
                div {
                    class = "article-tags",
                    article.tags.iter().enumerate(), => |i, tag| rsx!(span {
                        class = "tag",
                        @tag
                    })
                },
                p { @&article.content }
            })
        })
}

// Add route-specific content rendering
fn render_route_content(route: &Route) -> Element {
    // measure("route_render", || {
    match route {
        Route::Home => rsx!(div {
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
        }),
        Route::Articles => {
            render_articles(load_content())
        },
        Route::About => rsx!(div {
            class = "about-page",
            h1 { "About Us" },
            p { "This is the about page content." }
        }),
        Route::NotFound => rsx!(div {
            class = "not-found",
            h1 { "404 - Page Not Found" },
            p { "The page you're looking for doesn't exist." }
        })
    }
// })
}

// Add navigation menu
fn render_nav() -> Element {
    rsx!(nav {
        class = "main-nav",
        ul {
            li {
                click => move |_| navigate_to(Route::Home),
                "Home"
            },
            li {
                click => move |_| navigate_to(Route::Articles),
                "Articles"
            },
            li {
                click => move |_| navigate_to(Route::About),
                "About"
            },
            li {
                click => move |_| navigate_to(Route::NotFound),
                "Not Found"
            }
        }
    })
}


#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    main()
}

pub fn main() -> Result<(), JsValue> {
    let document = get_document();
    let body = document.body().expect("not found");

    // Set up popstate event listener for back/forward navigation
    let onpopstate = Closure::wrap(Box::new(move |_: web_sys::Event| {
        render();
    }) as Box<dyn FnMut(_)>);

    if let Some(window) = web_sys::window() {
        window.set_onpopstate(Some(onpopstate.as_ref().unchecked_ref()));
        onpopstate.forget();
    }

    STATE.with(|state| {
        let state = state.borrow();

        // let content = load_content();

        let current_route = get_current_route();

        // Pre-compute the class name
        let class_name = {
            let mut class = String::with_capacity(32); // Preallocate with reasonable size
            class.push_str("app-container ");
            class.push_str(state.theme.to_str());
            class
        };

        let app = rsx!(div {
            id = "app",
            class = class_name,
            render_nav(),
            render_route_content(&current_route),
            toggle_visibility(),
            counter_actions(),
            toggle_theme_button()
        });

        append_child!(body, &app);

        if let Some(root) = document.get_element_by_id("app") {
            state.theme_provider.apply_theme_to_element(&root)
                .expect("Failed to apply initial theme");
        }
    });

    Ok(())
}