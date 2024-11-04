use std::{cell::RefCell, sync::OnceLock};

use web_sys::js_sys;

pub static START_TIME: OnceLock<f64> = OnceLock::new();

fn get_start_time() -> f64 {
    *START_TIME.get_or_init(|| js_sys::Date::now())
}

pub fn time() -> f64 {
    js_sys::Date::now() - get_start_time()
}

thread_local! {
    static EVENT_TIMES: RefCell<Vec<(String, f64)>> = RefCell::new(Vec::new());
}

fn set_time(key: &str) {
    let current_time = time();
    EVENT_TIMES.with(|events| {
        events.borrow_mut().push((key.to_string(), current_time));
    });
}

fn get_last_event_time() -> Option<f64> {
    EVENT_TIMES.with(|events| events.borrow().last().map(|(_, time)| *time))
}

pub fn start_timed_event(key: &str, event: &str) {
    set_time(key);
    log_timed_event(&format!("Start for key {}: {}", key, event));
}

pub fn log_timed_event(event: &str) {
    let current_time = time();
    if let Some(last_time) = get_last_event_time() {
        log::debug!(
            "{}: '{}' Duration: {} ms",
            current_time,
            event,
            current_time - last_time
        );
    }
}
