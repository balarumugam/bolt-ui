use wasm_bindgen::prelude::*;
use web_sys::{window, console, History};
use crate::state::STATE;
use crate::log_stats;

#[derive(Clone, Debug, PartialEq)]
pub enum Route {
    Home,
    Articles,
    About,
    NotFound,
}

impl Route {
    pub fn from_path(path: &str) -> Self {
        match path {
            "/" | "" => Route::Home,
            "/articles" => Route::Articles,
            "/about" => Route::About,
            _ => Route::NotFound,
        }
    }

    pub fn to_path(&self) -> &'static str {
        match self {
            Route::Home => "/",
            Route::Articles => "/articles",
            Route::About => "/about",
            Route::NotFound => "/404",
        }
    }
}

pub fn navigate_to(route: Route) {
    if let Some(window) = window() {
        let history = window.history()
            .expect("Failed to get history")
            .dyn_into::<History>()
            .expect("Failed to cast history");

        let _ = history.push_state_with_url(
            &JsValue::NULL,
            "",
            Some(route.to_path())
        );

        STATE.with(|state| {
            let mut state = state.borrow_mut();
            state.current_route = route.clone();
        });
        
        // Manually trigger a render since pushState doesn't fire popstate
        crate::render();
    }

    // Log performance stats after navigation
    // log_stats();
}

pub fn get_current_route() -> Route {
    window()
        .and_then(|win| {
            win.document()
                .and_then(|doc| doc.location())
                .and_then(|loc| loc.pathname().ok())
        })
        .map(|path| Route::from_path(&path))
        .unwrap_or(Route::NotFound)
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    use web_sys::window;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_route_conversion_performance() {
        let test_cases = vec![
            ("", "Home"),
            ("/", "Home"),
            ("/articles", "Articles"),
            ("/about", "About"),
            ("/invalid", "NotFound"),
        ];
        let iterations = 10000;
        
        let window = window().expect("should have window");
        let performance = window.performance().expect("should have performance");

        console::group_1(&"Route Performance Metrics".into());
        
        // Test each route individually
        for (path, route_name) in test_cases {
            let start = performance.now();
            
            for _ in 0..iterations {
                let route = Route::from_path(path);
                let _path = route.to_path();
            }
            
            let duration = performance.now() - start;
            let ops_per_ms = iterations as f64 / duration;
            
            let message = format!(
                "Route '{}': {:.2}ms ({:.2} ops/ms) for {} iterations",
                route_name, duration, ops_per_ms, iterations
            );
            console::log_1(&message.into());
            
            assert!(
                duration < 50.0,
                "Route '{}' conversion took {:.2}ms ({:.2} ops/ms) for {} iterations",
                route_name, duration, ops_per_ms, iterations
            );
        }

        console::group_end();
    }

    #[wasm_bindgen_test]
    fn test_route_matching_accuracy() {
        let test_cases = vec![
            ("", Route::Home),
            ("/", Route::Home),
            ("/articles", Route::Articles),
            ("/about", Route::About),
            ("/invalid", Route::NotFound),
            ("/random", Route::NotFound),
        ];

        for (path, expected_route) in test_cases {
            let route = Route::from_path(path);
            assert_eq!(route, expected_route);
            
            // Test roundtrip conversion (except for NotFound cases)
            if route != Route::NotFound {
                assert_eq!(Route::from_path(route.to_path()), route);
            }
        }
    }
}