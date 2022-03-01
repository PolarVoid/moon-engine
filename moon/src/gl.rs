use web_sys::WebGl2RenderingContext;

pub type GL = WebGl2RenderingContext;

pub trait Bind {
    fn bind(&mut self, gl: &GL);
}