#[allow(dead_code)]
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub type Canvas = web_sys::HtmlCanvasElement;

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
    web_sys::window().unwrap().performance().unwrap().now() / 1000.0
}

pub fn setup_document_events() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    {
        let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            if event.is_composing() || event.key_code() == 229 {
                return;
            }
            panic!("Key was pressed {}", event.key());
        }) as Box<dyn FnMut(_)>);
        document.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    Ok(())
}
