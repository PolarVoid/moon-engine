mod utils;

use utils::set_panic_hook;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use nalgebra as na;
use nalgebra::{Matrix4, Vector3, Vector4, Projective3};
use web_sys::HtmlCanvasElement as Canvas;
use web_sys::WebGl2RenderingContext as GL;
use web_sys::{WebGlBuffer, WebGlVertexArrayObject, WebGlShader, WebGlProgram};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Creates a new `WebGlShader` using a source `str`.
pub fn create_shader(gl: &GL, shader_type: u32, source: &str) -> Result<WebGlShader, String> {
    let shader = gl.create_shader(shader_type).ok_or_else(|| String::from("Unable to create Shader object."))?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl.get_shader_parameter(&shader, GL::COMPILE_STATUS).as_bool().unwrap_or(false) {
        Ok(shader)
    } else {
        Err(gl.get_shader_info_log(&shader).unwrap_or_else(|| String::from("Could not create shader.")))
    }
}

pub fn create_program(gl: &GL, vertex_shader: &WebGlShader, fragment_shader: &WebGlShader) -> Result<WebGlProgram, String> {
    let program = gl.create_program().ok_or_else(|| String::from("Unable to create Program object."))?;
    gl.attach_shader(&program, &vertex_shader);
    gl.attach_shader(&program, &fragment_shader);
    gl.link_program(&program);
    
    if gl.get_program_parameter(&program, GL::LINK_STATUS).as_bool().unwrap_or(false) {
        Ok(program)
    } else {
        Err(gl.get_program_info_log(&program).unwrap_or_else(|| String::from("Could not create program.")))
    }
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

pub struct Transform {
    position: Vector3<f32>,
    scale: Vector3<f32>,
    rotation: Vector3<f32>,
    matrix: Matrix4<f32>,
}

impl Transform {
    pub fn new() -> Self{
        Transform {
            position: Vector3::from_element(0.0f32),
            scale: Vector3::from_element(1.0f32),
            rotation: Vector3::from_element(0.0f32),
            matrix: Matrix4::identity()
        }
    }
}

/// The `Vertex` struct
/// 
/// The `Vertex` struct holds the data that will be later sent to WebGL in a `GL::ARRAY_BUFFER`.
#[repr(C)]
struct Vertex {
    position: [f32; 3],
}

pub fn get_gl_context() -> Result<GL, String> {
    set_panic_hook();
    let document: web_sys::Document = web_sys::window().unwrap().document().unwrap();
    let canvas: Canvas = document.get_element_by_id("canvas").unwrap().dyn_into::<Canvas>().unwrap();
    let context: GL = canvas.get_context("webgl2").unwrap().unwrap().dyn_into::<GL>().unwrap();
    Ok(context)
}

#[wasm_bindgen]
pub struct Application {
    gl: GL,
    program: Option<WebGlProgram>,
}

#[wasm_bindgen]
impl Application {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            gl: get_gl_context().unwrap(),
            program: None,
        }
    }
    
    #[wasm_bindgen]
    pub fn init(&self) {
        let gl = &self.gl;
        gl.clear_color(0.0, 0.55, 0.7, 1.0);
        gl.clear(GL::COLOR_BUFFER_BIT|GL::DEPTH_BUFFER_BIT);
    
        let vertex_shader = create_shader(gl, GL::VERTEX_SHADER, include_str!("..\\res\\shader\\vertex.glsl")).expect("Could not create Vertex Shader!");
        let fragment_shader = create_shader(gl, GL::FRAGMENT_SHADER, include_str!("..\\res\\shader\\fragment.glsl")).expect("Could not create Fragment Shader!");
        
        let program = create_program(&gl, &vertex_shader, &fragment_shader).expect("Failed while creating Program!");
        gl.use_program(Some(&program));
        
        let position_attrib_location = gl.get_attrib_location(&program, "aPosition");
        let vao = gl.create_vertex_array().expect("Could not create Vertex Array Object.");
        gl.bind_vertex_array(Some(&vao));
        
        let vbo = gl.create_buffer().expect("Could not create Buffer Object.");
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vbo));
        
        let vertices = vec![
            Vertex {
                position: [-0.7, -0.7, 0.0],
            },
            Vertex {
                position: [0.7, -0.7, 0.0],
            },
            Vertex {
                position: [0.7, 0.7, 0.0],
            },
            Vertex {
                position: [0.7, 0.7, 0.0],
            },
            Vertex {
                position: [-0.7, 0.7, 0.0],
            },
            Vertex {
                position: [-0.7, -0.7, 0.0],
            },
        ];
    
        let u8_slice = unsafe {
            std::slice::from_raw_parts(
                vertices.as_ptr() as *const u8, vertices.len()*std::mem::size_of::<Vertex>()
            )
        };
        gl.buffer_data_with_u8_array(GL::ARRAY_BUFFER, u8_slice, GL::STATIC_DRAW);
    
        gl.vertex_attrib_pointer_with_i32(0, 3, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(position_attrib_location as u32);
        
        // gl.draw_arrays(GL::TRIANGLES, 0, 6);
        gl.enable(GL::DEPTH_TEST);
        gl.enable(GL::CULL_FACE);
    }

    #[wasm_bindgen]
    pub fn render(&self) {
        self.gl.clear(GL::COLOR_BUFFER_BIT|GL::DEPTH_BUFFER_BIT);
        self.gl.draw_arrays(GL::TRIANGLES, 0, 6);
    }
}