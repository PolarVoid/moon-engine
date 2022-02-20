#[allow(dead_code)]
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=console)]
    pub fn log(s: &str);
}

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (web::log(&format_args!($($t)*).to_string()))
}

// Get the time in seconds
pub fn now_sec() -> f64 {
    web_sys::window().unwrap()
        .performance().unwrap()
        .now() / 1000.0
}

