use crate::Canvas;
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext;

pub type GL = WebGl2RenderingContext;

pub trait Bind {
    fn bind(&self, gl: &GL);
    fn unbind(&self, _gl: &GL) {}
}

pub fn check_gl_error(gl: &GL) -> bool {
    let mut found_error = false;
    let mut gl_error = gl.get_error();
    while gl_error != GL::NO_ERROR {
        println!("OpenGL Error {}", gl_error);
        found_error = true;
        gl_error = gl.get_error();
    }
    found_error
}

pub fn get_context() -> GL {
    let document: web_sys::Document = web_sys::window().unwrap().document().unwrap();
    let canvas: Canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<Canvas>()
        .unwrap();
    let context: GL = canvas
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into::<GL>()
        .unwrap();
    context
}