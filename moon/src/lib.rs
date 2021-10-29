mod utils;
mod shader;
mod texture;

use na::Perspective3;
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use nalgebra as na;
use nalgebra::{Matrix4, Vector3, Vector4};
use web_sys::HtmlCanvasElement as Canvas;
use web_sys::WebGl2RenderingContext as GL;
use web_sys::{WebGlUniformLocation, HtmlImageElement};

use shader::create_shader;
use shader::create_program;
use texture::create_texture;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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
    color: [f32; 3],
    uv: [f32; 2],
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
    time: f32,
    u_time: Option<WebGlUniformLocation>,
    u_texture_0: Option<WebGlUniformLocation>,
    u_model_matrix: Option<WebGlUniformLocation>,
    u_view_matrix: Option<WebGlUniformLocation>,
    u_projection_matrix: Option<WebGlUniformLocation>,
}

#[wasm_bindgen]
impl Application {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            gl: get_gl_context().unwrap(),
            time: 0.0,
            u_time: None,
            u_texture_0: None,
            u_model_matrix: None,
            u_view_matrix: None,
            u_projection_matrix: None,
        }
    }
    
    #[wasm_bindgen]
    pub fn init(&mut self) {
        let gl = &self.gl;
        gl.clear_color(0.0, 0.55, 0.7, 1.0);
        gl.clear(GL::COLOR_BUFFER_BIT|GL::DEPTH_BUFFER_BIT);

        let vertex_shader = create_shader(gl, GL::VERTEX_SHADER, include_str!("..\\res\\shader\\vertex.glsl")).expect("Could not create Vertex Shader!");
        let fragment_shader = create_shader(gl, GL::FRAGMENT_SHADER, include_str!("..\\res\\shader\\fragment.glsl")).expect("Could not create Fragment Shader!");
        
        let program = create_program(&gl, &vertex_shader, &fragment_shader).expect("Failed while creating Program!");
        gl.use_program(Some(&program));
        
        gl.delete_shader(Some(&vertex_shader));
        gl.delete_shader(Some(&fragment_shader));

        let u_time = gl.get_uniform_location(&program, "uTime");
        self.u_time = u_time;
        let u_texture_0 = gl.get_uniform_location(&program, "uTex0");
        self.u_texture_0 = u_texture_0;

        let u_model_matrix = gl.get_uniform_location(&program, "uModel");
        self.u_model_matrix = u_model_matrix;
        let u_view_matrix = gl.get_uniform_location(&program, "uView");
        self.u_view_matrix = u_view_matrix;
        let u_projection_matrix = gl.get_uniform_location(&program, "uProj");
        self.u_projection_matrix = u_projection_matrix;

        let position_attrib_location = gl.get_attrib_location(&program, "aPosition");
        let vcolor_attrib_location = gl.get_attrib_location(&program, "aColor");
        let uv_attrib_location = gl.get_attrib_location(&program, "aTexCoord");
        
        let vao = gl.create_vertex_array().expect("Could not create Vertex Array Object.");
        gl.bind_vertex_array(Some(&vao));
        
        let vbo = gl.create_buffer().expect("Could not create Buffer Object.");
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vbo));

        let ibo = gl.create_buffer().expect("Could not create Index Buffer Object.");
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&ibo));
        
        let vertices = vec![
            Vertex {
                position: [-0.5, 0.0, 0.5],
                color: [0.7, 0.3, 0.7],
                uv: [0.0, 0.0],
            },
            Vertex {
                position: [-0.5, 0.0, -0.5],
                color: [0.5, 0.2, 0.0],
                uv: [5.0, 0.0],
            },
            Vertex {
                position: [0.5, 0.0, -0.5],
                color: [0.8, 0.6, 0.0],
                uv: [0.0, 0.0],
            },
            Vertex {
                position: [0.5, 0.0, 0.5],
                color: [0.0, 0.4, 0.8],
                uv: [5.0, 5.0],
            },
            Vertex {
                position: [0.0, 0.8, 0.0],
                color: [0.0, 0.4, 0.8],
                uv: [2.5, 5.0],
            },
        ];
        
        let indices : [u8; 18] = [
            0, 1, 2,
            0, 2, 3,
            0, 1, 4,
            1, 2, 4,
            2, 3, 4,
            3, 0, 4,
            ];
        let u8_slice = unsafe {
            std::slice::from_raw_parts(
                vertices.as_ptr() as *const u8, vertices.len()*std::mem::size_of::<Vertex>()
            )
        };
        gl.buffer_data_with_u8_array(GL::ARRAY_BUFFER, u8_slice, GL::STATIC_DRAW);
        gl.buffer_data_with_u8_array(GL::ELEMENT_ARRAY_BUFFER, &indices, GL::STATIC_DRAW);
    
        gl.vertex_attrib_pointer_with_i32(0, 3, GL::FLOAT, false, 8 * 4, 0);
        gl.vertex_attrib_pointer_with_i32(1, 3, GL::FLOAT, false, 8 * 4, 12);
        gl.vertex_attrib_pointer_with_i32(2, 2, GL::FLOAT, false, 8 * 4, 24);

        gl.enable_vertex_attrib_array(position_attrib_location as u32);
        gl.enable_vertex_attrib_array(vcolor_attrib_location as u32);
        gl.enable_vertex_attrib_array(uv_attrib_location as u32);

        let document: web_sys::Document = web_sys::window().unwrap().document().unwrap();
        let img = document.get_element_by_id("texture0").unwrap().dyn_into::<HtmlImageElement>().unwrap();
        let texture = create_texture(gl, &img).expect("Failed to create Texture");

        let model = Matrix4::<f32>::identity();
        let view = Matrix4::<f32>::new_translation(&Vector3::new(0.0, -0.5, -2.0));
        let proj: Perspective3<f32> = Perspective3::new(16.0/9.0, 3.14 / 4.0, 0.1, 1000.0);
        gl.uniform1i(self.u_texture_0.as_ref(), 0);
        gl.uniform_matrix4fv_with_f32_array(self.u_model_matrix.as_ref(), false, model.as_slice());
        gl.uniform_matrix4fv_with_f32_array(self.u_view_matrix.as_ref(), false, view.as_slice());
        gl.uniform_matrix4fv_with_f32_array(self.u_projection_matrix.as_ref(), false, proj.as_matrix().as_slice());
        gl.enable(GL::DEPTH_TEST);
    }

    #[wasm_bindgen]
    pub fn render(&mut self) {
        let gl = &self.gl;
        self.time += 0.1;
        gl.clear(GL::COLOR_BUFFER_BIT|GL::DEPTH_BUFFER_BIT);
        gl.uniform1f(self.u_time.as_ref(), self.time);
        gl.draw_elements_with_i32(GL::TRIANGLES, 18, GL::UNSIGNED_BYTE, 0);
    }
}