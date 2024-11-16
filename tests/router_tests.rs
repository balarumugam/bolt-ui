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