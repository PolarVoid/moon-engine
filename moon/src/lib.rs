#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]

//! Moon Engine

pub mod camera;
pub mod collider;
pub mod component;
pub mod gl;
pub mod input;
pub mod math;
pub mod mesh;
pub mod particle;
pub mod renderer;
pub mod shader;
pub mod texture;
pub mod transform;
pub mod ui;
pub mod utils;
pub mod web;

use wasm_bindgen::prelude::*;

use camera::Camera;
use gl::GL;
use input::InputManager;
pub use math::*;
use renderer::Renderer;
use shader::Shader;
use transform::Transform;
use utils::set_panic_hook;
use web::Canvas;

use crate::component::Component;
use crate::particle::ParticleSystem;
use crate::texture::Texture;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// The [`Application`] struct acts as the communicator between the browser and the game logic. It consists of calls made from JavaScript.
#[wasm_bindgen]
pub struct Application {
    renderer: Renderer,
    input: InputManager,
}

impl Default for Application {
    fn default() -> Self {
        // Initialize the JS panic hook
        set_panic_hook();
        Self {
            renderer: Renderer::default(),
            input: InputManager::new(),
        }
    }
}

#[allow(clippy::unused_unit)]
#[wasm_bindgen]
impl Application {
    /// Initilize a default [`Application`].
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set up data before render loop.
    #[wasm_bindgen]
    pub fn init(&mut self) {
        let renderer = &mut self.renderer;

        renderer
            .gl
            .blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);
        renderer.gl.enable(GL::BLEND);
        // Initialize the default Shader
        renderer.init_shader();

        let u_tex0 = renderer.program.get_uniform_location(&renderer.gl, "uTex0");
        renderer.gl.uniform1i(u_tex0.as_ref(), 0);

        let spritesheet = Texture::new_with_texture_id(&renderer.gl, 0);

        renderer.add_texture("PLATFORMER", spritesheet);

        renderer.use_texture("WHITE");

        let mut test = ParticleSystem::default();
        test.init();
        renderer.add_component("PARTICLE", Box::new(test));

    }

    /// Called when window gets resized.
    #[wasm_bindgen]
    pub fn resize(&mut self, width: f32, height: f32) {
        self.renderer.resize(width, height);
    }

    /// Called when a keyboard input event is generated.
    #[wasm_bindgen]
    pub fn input(&mut self, key_code: u8, is_down: bool) {
        if is_down {
            self.input.key_down(key_code);
        } else {
            self.input.key_up(key_code);
        }
    }

    /// Handles Mouse movement.
    #[wasm_bindgen]
    pub fn mouse_move(&mut self, _mouse_x: i32, _mouse_y: i32) {
        //     let (x, y) = self
        //         .camera
        //         .screen_to_world_coordinates(mouse_x as f32, mouse_y as f32);
        //     //self.objects[1].transform.position = Vector3::new(x, 0.0, y);
    }

    /// Renders a new frame.
    ///
    /// Called every frame, and draws its output onto the [Canvas](web_sys::HtmlCanvasElement).
    #[wasm_bindgen]
    pub fn render(&mut self, delta_time: u32) {
        let delta_time = delta_time as f32 / 1000.0;

        self.renderer.clear([0.5, 0.2, 0.3, 1.0]);
        
        if self.input.get_key_state(b'R') {
            self.renderer.components.get_mut("PARTICLE").unwrap().init();
        }
        let ps = self.renderer.components.get_mut("PARTICLE").unwrap().as_mut_any().downcast_mut::<ParticleSystem>().unwrap();
        ps.emit_many(5);

        self.renderer.update_components(delta_time);

        self.renderer.draw_components();

        // self.renderer.begin_layer();
        // self.renderer.add_quad(Quad::default());
        // self.renderer.use_texture("MAGENTA");
        // self.renderer.draw_layer();
        // self.renderer.delete_layer();
    }
}
