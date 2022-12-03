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
use particle::ParticleSystem;
use renderer::Renderer;
use shader::Shader;
use texture::Texture;
use transform::Transform;
use utils::set_panic_hook;
use web::Canvas;

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

        // Initialize global WebGL state
        renderer
            .gl
            .blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);
        renderer.gl.enable(GL::BLEND);

        // Initialize the default Shader
        renderer.init_shader();

        // Obtain Texture Slot 0 uniform location
        let u_tex0 = renderer.program.get_uniform_location(&renderer.gl, "uTex0");
        renderer.gl.uniform1i(u_tex0.as_ref(), 0);

        // Add default Textures to the Renderer list
        renderer.add_texture("TILEMAP", Texture::new_with_texture_id(&renderer.gl, 0));
        renderer.add_texture("SHREK", Texture::new_with_texture_id(&renderer.gl, 1));

        // Use a 1x1 pixel "WHITE" texture
        renderer.use_texture("WHITE");

        let simple = ParticleSystem::new_from_emission_and_position(
            particle::ParticleProps::default(),
            0.0,
            0.0,
        );
        renderer.add_component("DEFAULT", Box::new(simple));

        let fire = ParticleSystem::new_from_emission_and_position(
            particle::ParticleProps::fire(),
            -5.0,
            0.0,
        );
        renderer.add_component("FIRE", Box::new(fire));

        let smoke = ParticleSystem::new_from_emission_and_position(
            particle::ParticleProps::smoke(),
            5.0,
            0.0,
        );
        renderer.add_component("SMOKE", Box::new(smoke));

        // Initialize Renderer components
        renderer.init_components();
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
    pub fn mouse_move(&mut self, mouse_x: i32, mouse_y: i32) {
        let (x, y) = self
            .renderer
            .camera
            .screen_to_world_coordinates(mouse_x as f32, mouse_y as f32);
        self.input.mouse_position = Vec2::new(x, y);
    }

    /// Renders a new frame.
    ///
    /// Called every frame, and draws its output onto the [Canvas](web_sys::HtmlCanvasElement).
    #[wasm_bindgen]
    pub fn render(&mut self, delta_time: u32) {
        let renderer = &mut self.renderer;
        let delta_time = delta_time as f32 / 1000.0;

        // Clear the screen
        renderer.clear([0.5, 0.2, 0.3, 1.0]);

        // Reset all Components to their initial state
        if self.input.consume_key(b'R') {
            renderer.init_components();
        }

        // Pause and Play the "FIRE" particle system
        if self.input.consume_key(b'1') {
            renderer
                .get_mut_component::<ParticleSystem>("FIRE")
                .unwrap()
                .toggle_alive();
        }

        // Pause and Play the "DEFAULT" particle system
        if self.input.consume_key(b'2') {
            renderer
                .get_mut_component::<ParticleSystem>("DEFAULT")
                .unwrap()
                .toggle_alive();
        }

        // Pause and Play the "SMOKE" particle system
        if self.input.consume_key(b'3') {
            renderer
                .get_mut_component::<ParticleSystem>("SMOKE")
                .unwrap()
                .toggle_alive();
        }

        // Get the horizontal and vertical axes of movement using the WASD keys
        let horizontal =
            self.input.get_key_state(b'D') as i32 - self.input.get_key_state(b'A') as i32;
        let vertical =
            self.input.get_key_state(b'S') as i32 - self.input.get_key_state(b'W') as i32;

        // Get a mutable reference to the "DEFAULT" particle system
        let simple = renderer
            .get_mut_component::<ParticleSystem>("DEFAULT")
            .unwrap();

        // Update the position of the "DEFAULT" particle system
        simple.transform.position = self.input.mouse_position;

        // Get a mutable reference to the "SMOKE" particle system
        let smoke = renderer
            .get_mut_component::<ParticleSystem>("SMOKE")
            .unwrap();

        // Update the position of the "SMOKE" particle system if it is alive
        if smoke.alive {
            smoke.transform.position +=
                Vec2::new(horizontal as f32 * delta_time, vertical as f32 * delta_time);
        }

        // Call the update() function on all components
        renderer.update_components(delta_time);

        // Render all components on-screen by issuing the draw call(s)
        renderer.draw_components();

        // self.renderer.begin_layer();
        // self.renderer.add_quad(&renderer::Quad::default());
        // self.renderer.use_texture("MAGENTA");
        // self.renderer.draw_layer();
        // self.renderer.delete_layer();
    }
}
