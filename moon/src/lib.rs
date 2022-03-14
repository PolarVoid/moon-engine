pub mod camera;
pub mod collider;
pub mod gl;
pub mod input;
pub mod math;
pub mod mesh;
pub mod particle;
pub mod shader;
pub mod texture;
pub mod transform;
pub mod utils;
pub mod web;

use wasm_bindgen::prelude::*;

use camera::Camera;
use gl::Renderer;
use gl::GL;
use input::InputManager;
pub use math::*;
use mesh::Mesh;
use shader::Shader;
use texture::Texture;
use transform::Transform;
use utils::set_panic_hook;
use web::Canvas;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Default)]
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
        let u_tex0 = self
            .renderer
            .program
            .get_uniform_location(&self.renderer.gl, "uTex0");
        self.renderer.gl.uniform1i(u_tex0.as_ref(), 0);
        self.renderer.begin_draw();

        for x_offset in -10..=10 {
            for y_offset in -6..=6 {
                let mut transform = Transform::new_with_position(nalgebra::Vector3::new(
                    x_offset as f32,
                    y_offset as f32,
                    0.0,
                ));
                transform.set_scale(nalgebra::Vector3::new(1.0, 1.0, 1.0));
                let quad = gl::Quad::new_from_transform(transform);
                self.renderer.add_quad(quad);
            }
        }
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
    #[wasm_bindgen]
    pub fn render(&mut self, _delta_time: u32) {
        use nalgebra::Vector3;
        self.renderer.clear([0.5, 0.2, 0.3, 1.0]);

        let position = [
            self.input.get_key_state(b'D') as i32 - self.input.get_key_state(b'A') as i32,
            self.input.get_key_state(b'S') as i32 - self.input.get_key_state(b'W') as i32,
        ];
        let position = Vector3::new(position[0] as f32, position[1] as f32, 0.0)
            / (_delta_time as f32 * 100.0)
            * 15.0;

        self.renderer
            .camera
            .transform
            .set_position(self.renderer.camera.transform.position + position);

        self.renderer.use_texture("CHECKERBOARD");
        self.renderer.end_draw();
        self.renderer.begin_layer();
        self.renderer.add_quad(gl::Quad::default());
        self.renderer.use_texture("MAGENTA");
        self.renderer.draw_layer();
        self.renderer.delete_layer();
        //self.renderer.end_draw();
    }
}
