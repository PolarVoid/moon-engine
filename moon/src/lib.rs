pub mod camera;
pub mod collider;
pub mod gl;
pub mod input;
pub mod mesh;
pub mod particle;
pub mod shader;
pub mod texture;
pub mod transform;
pub mod utils;
pub mod web;

use {
    wasm_bindgen::{prelude::*},
};

pub use camera::Camera;
pub use collider::Circle;
pub use collider::Collide;
pub use collider::AABB;
use gl::Renderer;
use gl::GL;
pub use input::InputManager;
pub use mesh::Mesh;
pub use shader::Shader;
pub use texture::Texture;
pub use transform::Transform;
use utils::set_panic_hook;
use web::Canvas;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Application {
    renderer: Renderer,
    input: InputManager,
}

#[wasm_bindgen]
impl Application {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        // Initialize the JS panic hook
        set_panic_hook();
        Self {
            renderer: Renderer::default(),
            input: InputManager::new(),
        }
    }

    #[wasm_bindgen]
    pub fn init(&mut self) {
        self.renderer.init_shader();
        // let mesh = Mesh::primitive(gl, Shape::Quad(1.0));
        // mesh.setup(gl);
        // gl.enable_vertex_attrib_array(position_attrib_location as u32);
        // gl.enable_vertex_attrib_array(uv_attrib_location as u32);
        // gl.enable_vertex_attrib_array(normal_attrib_location as u32);

        // let document: web_sys::Document = web_sys::window().unwrap().document().unwrap();
        // let image = document
        //     .get_element_by_id("texture0")
        //     .unwrap()
        //     .dyn_into::<web_sys::HtmlImageElement>()
        //     .unwrap();
        // let texture = Texture::new(gl, &image);

        // texture.bind(gl);

        // let img2 = document
        //     .get_element_by_id("texture1")
        //     .unwrap()
        //     .dyn_into::<HtmlImageElement>()
        //     .unwrap();
        // let _texture_spec = create_texture(gl, &img2, 1).expect("Failed to create Texture");

        // let mut initial_camera_transform = Transform::default();
        // initial_camera_transform.rotation = 0.0;
        // self.camera = Camera::with_transform(initial_camera_transform);
        // let model: Matrix4<f32> = Matrix4::identity();
        // gl.uniform1i(u_texture_0.as_ref(), 0);
        // gl.uniform1i(u_texture_1.as_ref(), 0);
        // gl.uniform4f(self.u_color.as_ref(), 1.0, 1.0, 1.0, 1.0);
        // gl.uniform_matrix4fv_with_f32_array(self.u_model_matrix.as_ref(), false, model.as_slice());
        
    }

    #[wasm_bindgen]
    pub fn resize(&mut self, width: f32, height: f32) {
        self.renderer.resize(width, height);
    }

    #[wasm_bindgen]
    pub fn input(&mut self, key_code: u8, is_down: bool) {
        if is_down {
            self.input.key_down(key_code);
        } else {
            self.input.key_up(key_code);
        }
    }

    #[allow(dead_code, unused_variables)]
    #[wasm_bindgen]
    pub fn mouse_move(&mut self, mouse_x: i32, mouse_y: i32) {
    //     let (x, y) = self
    //         .camera
    //         .screen_to_world_coordinates(mouse_x as f32, mouse_y as f32);
    //     //self.objects[1].transform.position = Vector3::new(x, 0.0, y);
    }
    #[allow(dead_code, unused_variables, unused_assignments)]
    #[wasm_bindgen]
    pub fn render(&mut self, delta_time: u32) {
        self.renderer.clear([0.5, 0.2, 0.3, 1.0]);
        self.renderer.begin_draw();
        self.renderer.add_quad(gl::Quad::new_from_transform(Transform::new_with_position(nalgebra::Vector3::new(0.0, 0.0, 0.0))));
        self.renderer.add_quad(gl::Quad::new_from_transform(Transform::new_with_position(nalgebra::Vector3::new(2.0, 3.0, 0.0))));
        self.renderer.add_quad(gl::Quad::new_from_transform(Transform::new_with_position(nalgebra::Vector3::new(2.0, 3.0, 0.0))));
        self.renderer.add_quad(gl::Quad::new_from_transform(Transform::new_with_position(nalgebra::Vector3::new(2.0, 3.0, 0.0))));
        self.renderer.end_draw();
    }
}
