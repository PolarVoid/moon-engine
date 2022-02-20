pub mod camera;
pub mod web;
pub mod input;
pub mod mesh;
pub mod shader;
pub mod texture;
pub mod transform;
pub mod collider;
mod utils;

use {
    nalgebra::{
        Matrix4,
        Vector2
    },
    wasm_bindgen::{
        prelude::*, 
        JsCast
    },
    web_sys::{
        HtmlCanvasElement,
        WebGl2RenderingContext,
        HtmlImageElement,
        WebGlUniformLocation
    }
};

pub use camera::Camera;
pub use input::InputManager;
pub use mesh::Mesh;
pub use mesh::Shape;
pub use mesh::Vertex;
pub use shader::create_program;
pub use shader::create_shader;
pub use texture::create_texture;
pub use transform::Transform;
pub use collider::AABB;
pub use collider::Circle;
pub use collider::Collide;
use utils::set_panic_hook;

type Canvas = HtmlCanvasElement;
type GL = WebGl2RenderingContext;

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

pub fn get_gl_context() -> Result<GL, String> {
    set_panic_hook();
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
    Ok(context)
}

#[wasm_bindgen]
pub struct Application {
    gl: GL,
    camera: Camera,
    input: InputManager,
    u_time: Option<WebGlUniformLocation>,
    u_color: Option<WebGlUniformLocation>,
    u_model_matrix: Option<WebGlUniformLocation>,
    u_view_matrix: Option<WebGlUniformLocation>,
    u_projection_matrix: Option<WebGlUniformLocation>,
    u_camera_position: Option<WebGlUniformLocation>,
}

#[wasm_bindgen]
impl Application {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            gl: get_gl_context().unwrap(),
            camera: Camera::new(),
            input: InputManager::new(),
            u_time: None,
            u_color: None,
            u_model_matrix: None,
            u_view_matrix: None,
            u_projection_matrix: None,
            u_camera_position: None,
        }
    }

    #[wasm_bindgen]
    pub fn init(&mut self) {
        console_log!("Hello, world!");
        let gl = &self.gl;
        gl.clear_color(0.0, 0.11, 0.2, 1.0);
        gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        let vertex_shader = create_shader(
            gl,
            GL::VERTEX_SHADER,
            include_str!("../res/shader/default.vert.glsl"),
        )
        .expect("Could not create Vertex Shader!");
        let fragment_shader = create_shader(
            gl,
            GL::FRAGMENT_SHADER,
            include_str!("../res/shader/default.frag.glsl"),
        )
        .expect("Could not create Fragment Shader!");

        let program = create_program(gl, &vertex_shader, &fragment_shader)
            .expect("Failed while creating Program!");
        gl.use_program(Some(&program));

        gl.delete_shader(Some(&vertex_shader));
        gl.delete_shader(Some(&fragment_shader));

        let u_time = gl.get_uniform_location(&program, "uTime");
        self.u_time = u_time;
        let u_texture_0 = gl.get_uniform_location(&program, "uTex0");
        let u_texture_1 = gl.get_uniform_location(&program, "uTex1");
        let u_color = gl.get_uniform_location(&program, "uColor");
        let u_model_matrix = gl.get_uniform_location(&program, "uModel");
        self.u_model_matrix = u_model_matrix;
        let u_view_matrix = gl.get_uniform_location(&program, "uView");
        self.u_view_matrix = u_view_matrix;
        let u_projection_matrix = gl.get_uniform_location(&program, "uProj");
        self.u_projection_matrix = u_projection_matrix;

        let u_camera_position = gl.get_uniform_location(&program, "uCamPos");
        self.u_camera_position = u_camera_position;

        let position_attrib_location = gl.get_attrib_location(&program, "aPosition");
        let uv_attrib_location = gl.get_attrib_location(&program, "aTexCoord");
        let normal_attrib_location = gl.get_attrib_location(&program, "aNormal");

       
        let mesh = Mesh::primitive(gl, Shape::Quad(1.0));
        mesh.setup(gl);
        gl.enable_vertex_attrib_array(position_attrib_location as u32);
        gl.enable_vertex_attrib_array(uv_attrib_location as u32);
        gl.enable_vertex_attrib_array(normal_attrib_location as u32);
        
        let mesh = Mesh::primitive(gl, Shape::Quad(1.0));
        mesh.setup(gl);
        gl.enable_vertex_attrib_array(position_attrib_location as u32);
        gl.enable_vertex_attrib_array(uv_attrib_location as u32);
        gl.enable_vertex_attrib_array(normal_attrib_location as u32);

        let document: web_sys::Document = web_sys::window().unwrap().document().unwrap();
        let img1 = document
            .get_element_by_id("texture0")
            .unwrap()
            .dyn_into::<HtmlImageElement>()
            .unwrap();
        let _texture_alb = create_texture(gl, &img1, 0).expect("Failed to create Texture");
        let img2 = document
            .get_element_by_id("texture1")
            .unwrap()
            .dyn_into::<HtmlImageElement>()
            .unwrap();
        let _texture_spec = create_texture(gl, &img2, 1).expect("Failed to create Texture");

        let mut initial_camera_transform = Transform::new_with_position(-Vector2::y() * 1.0);
        initial_camera_transform.rotation = 0.0;
        self.camera = Camera::with_transform(initial_camera_transform);
        let model: Matrix4<f32> = Matrix4::identity();

        gl.uniform1i(u_texture_0.as_ref(), 0);
        gl.uniform1i(u_texture_1.as_ref(), 0);
        gl.uniform4f(u_color.as_ref(), 1.0, 1.0, 1.0, 1.0);
        gl.uniform_matrix4fv_with_f32_array(self.u_model_matrix.as_ref(), false, model.as_slice());
        gl.uniform_matrix4fv_with_f32_array(
            self.u_view_matrix.as_ref(),
            false,
            self.camera.transform.matrix(),
        );
        gl.uniform_matrix4fv_with_f32_array(
            self.u_projection_matrix.as_ref(),
            false,
            self.camera.projection(),
        );
        self.u_color = u_color;
        gl.enable(GL::CULL_FACE);
    }

    #[wasm_bindgen]
    pub fn resize(&mut self, width: f32, height: f32) {
        self.camera.set_width_and_height(width, height);
        self.gl.viewport(0, 0, width as i32, height as i32);
        self.gl.uniform_matrix4fv_with_f32_array(
            self.u_projection_matrix.as_ref(),
            false,
            self.camera.projection(),
        );
    }

    #[wasm_bindgen]
    pub fn input(&mut self, key_code: u8, is_down: bool) {
        if is_down {
            self.input.key_down(key_code);
        } else {
            self.input.key_up(key_code);
        }
    }

    #[wasm_bindgen]
    pub fn mouse_move(&mut self, mouse_x: i32, mouse_y: i32) {
        let (x, y) = self.camera.screen_to_world_coordinates(mouse_x as f32, mouse_y as f32);
        //self.objects[1].transform.position = Vector3::new(x, 0.0, y);
    }
    #[wasm_bindgen]
    pub fn render(&mut self, delta_time: u32) {
        let speed = 5f32;
        let gl = &self.gl;
        let mut horizontal_axis = 0.0f32;
        let mut vertical_axis = 0.0f32;
        if self.input.get_key_state('A' as u8) {
            horizontal_axis += 1.0;
        }
        if self.input.get_key_state('D' as u8) {
            horizontal_axis -= 1.0;
        }
        if self.input.get_key_state('W' as u8) {
            vertical_axis += 1.0;
        }
        if self.input.get_key_state('S' as u8) {
            vertical_axis -= 1.0;
        }
        // self.objects[0].transform.position.y = 0.0;
        // self.objects[0].transform.position -= (Vector3::x() * horizontal_axis + Vector3::z() * vertical_axis) * speed * (delta_time as f32 / 1000.0);
        // self.objects[0].transform.position.x = nalgebra::clamp(self.objects[0].transform.position.x, -5.0, 5.0);
        // let box1 = Circle::new_position(self.objects[0].transform.position.x, self.objects[0].transform.position.z);
        // let box2 = Circle::new_position(self.objects[1].transform.position.x, self.objects[1].transform.position.z);
        // if box2.collide_with(&box1) {
        //     gl.uniform4f(self.u_color.as_ref(), 1.0, 0.0, 0.0, 1.0);
        // } else {
        //     gl.uniform4f(self.u_color.as_ref(), 0.0, 1.0, 0.0, 1.0);
        // }
        gl.clear(GL::COLOR_BUFFER_BIT);
        gl.uniform3fv_with_f32_array(
            self.u_camera_position.as_ref(),
            self.camera.transform.get_position(),
        );
        gl.uniform_matrix4fv_with_f32_array(
            self.u_view_matrix.as_ref(),
            false,
            self.camera.transform.matrix(),
        );
        gl.uniform1f(self.u_time.as_ref(), delta_time as f32 * 0.001);
        
    }
}
