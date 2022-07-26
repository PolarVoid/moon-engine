//! Web related utilities and the [`Canvas`] alias to [web_sys::HtmlCanvasElement]

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

/// An alias for [`web_sys::HtmlCanvasElement`]
pub type Canvas = web_sys::HtmlCanvasElement;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=console)]
    pub fn log(s: &str);
}

/// A macro that can be used to write output to the browser
#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => ($crate::web::log(&format_args!($($t)*).to_string()))
}

/// Get the time in seconds using [`Performance`](web_sys::Performance)
///
/// # Examples
///
/// ```no_run
/// # use moon_engine::web::now_sec;
/// let time = now_sec();
/// println!("The current time is: {}", time);
/// ```
pub fn now_sec() -> f64 {
    web_sys::window().unwrap().performance().unwrap().now() / 1000.0
}

/// Initialize document-level callbacks
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
