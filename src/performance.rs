use web_sys::Performance;
use std::collections::HashMap;
use std::cell::RefCell;

// Performance metric types
#[derive(Debug, Clone)]
pub struct MetricStats {
    pub avg: f64,
    pub min: f64,
    pub max: f64,
    pub samples: usize,
}

thread_local! {
    static PERFORMANCE_METRICS: RefCell<HashMap<&'static str, Vec<f64>>> = RefCell::new(HashMap::new());
}

pub fn measure<F, T>(name: &'static str, f: F) -> T 
where 
    F: FnOnce() -> T 
{
    let window = web_sys::window().expect("no window");
    let performance = window
        .performance()
        .expect("performance should be available");
    
    let start = performance.now();
    let result = f();
    let end = performance.now();
    let duration = end - start;

    // Store the measurement
    PERFORMANCE_METRICS.with(|metrics| {
        let mut metrics = metrics.borrow_mut();
        metrics
            .entry(name)
            .or_insert_with(Vec::new)
            .push(duration);
    });

    // Log the current measurement
    web_sys::console::log_1(&format!("🔍 {} took {:.2}ms", name, duration).into());
    
    result
}

pub fn get_stats(name: &'static str) -> Option<MetricStats> {
    PERFORMANCE_METRICS.with(|metrics| {
        let metrics = metrics.borrow();
        metrics.get(name).map(|durations| {
            let len = durations.len();
            if len == 0 {
                return MetricStats {
                    avg: 0.0,
                    min: 0.0,
                    max: 0.0,
                    samples: 0,
                };
            }

            let sum: f64 = durations.iter().sum();
            let min = durations.iter().fold(f64::INFINITY, |a, &b| a.min(b));
            let max = durations.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

            MetricStats {
                avg: sum / len as f64,
                min,
                max,
                samples: len,
            }
        })
    })
}

pub fn log_stats() {
    PERFORMANCE_METRICS.with(|metrics| {
        let metrics = metrics.borrow();
        for (name, durations) in metrics.iter() {
            if !durations.is_empty() {
                let stats = get_stats(name).unwrap();
                web_sys::console::log_1(&format!(
                    "📊 {} stats:\n  Avg: {:.2}ms\n  Min: {:.2}ms\n  Max: {:.2}ms\n  Samples: {}",
                    name, stats.avg, stats.min, stats.max, stats.samples
                ).into());
            }
        }
    });
}