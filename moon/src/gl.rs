//! A collection of functions and traits related to [`WebGl2RenderingContext`], as well as the [`GL`] alias.

use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext;

use crate::Canvas;

/// An alias for [`WebGl2RenderingContext`].
///
/// # Examples
///
/// ```no_run
/// use moon::*;
/// let gl = gl::get_context();
/// ```
pub type GL = WebGl2RenderingContext;

/// The [`Bind`] trait enables setting up WebGl state when bound, and optionally resetting to a neutral one when unbound.
///
/// These functons should be implented **without** mutating the implementing struct, i.e. just changing the state of the [`WebGl2RenderingContext`].
pub trait Bind {
    /// Binds a struct implementing the [`Bind`] trait. This is up to the implementation to decide.
    fn bind(&self, gl: &GL);
    /// Optionally unbinds a struct implementing the [`Bind`] trait. This effectively resets the WebGL state.
    fn unbind(&self, _gl: &GL) {}
}

/// Check for, and print any WebGL errors if found.
///
/// Takes a reference to a [`WebGl2RenderingContext`] and returns a [`bool`], indicating whether any errors were found.
///
/// # Examples
///
/// ```no_run
/// use moon::*;
/// # let gl = gl::get_context();
///
/// let has_errors = gl::check_gl_error(&gl);
/// ```
///
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

/// Get the `WebGl2RenderingContext` of a canvas with an *element ID* of **"canvas"**
///
/// This function will panic in case of an error
///
/// # Examples
///
/// ```no_run
/// use moon::gl::*;
///
/// let context: GL = get_context();
/// ```
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
