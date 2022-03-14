use std::collections::BTreeMap;

use crate::{
    mesh::{Vertex, MAX_BATCH_INDICES, MAX_BATCH_VERTICES},
    texture::SubTexture,
    Camera, Canvas, Mesh, Shader, Texture, Transform, Vec2,
};
use wasm_bindgen::JsCast;
use web_sys::{WebGl2RenderingContext, WebGlUniformLocation};

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

#[derive(Debug)]
pub struct Renderer {
    pub gl: GL,
    pub program: Shader,
    pub camera: Camera,
    batches: Vec<Mesh>,
    textures: BTreeMap<&'static str, Texture>,
    u_time: Option<WebGlUniformLocation>,
    u_color: Option<WebGlUniformLocation>,
    u_model_matrix: Option<WebGlUniformLocation>,
    u_view_matrix: Option<WebGlUniformLocation>,
    u_projection_matrix: Option<WebGlUniformLocation>,
}

impl Default for Renderer {
    fn default() -> Self {
        let gl = get_context();
        let program = Shader::new(&gl);
        Self {
            camera: Camera::default(),
            batches: Vec::new(),
            u_time: program.get_uniform_location(&gl, "uTime"),
            u_color: program.get_uniform_location(&gl, "uColor"),
            u_model_matrix: program.get_uniform_location(&gl, "uModel"),
            u_view_matrix: program.get_uniform_location(&gl, "uView"),
            u_projection_matrix: program.get_uniform_location(&gl, "uProj"),
            program,
            textures: {
                let mut textues = BTreeMap::<&str, Texture>::new();
                textues.insert("WHITE", Texture::white(&gl));
                textues.insert("MAGENTA", Texture::colored(&gl, crate::MAGENTA));
                textues.insert("CHECKERBOARD", Texture::checkerboard(&gl));
                textues
            },
            gl,
        }
    }
}

#[derive(Debug)]
pub struct Quad {
    pub points: [Vec2; 4],
    pub sprite: SubTexture,
}

impl Default for Quad {
    fn default() -> Self {
        Self {
            points: [
                Vec2::new(-0.5, 0.5),
                Vec2::new(-0.5, -0.5),
                Vec2::new(0.5, -0.5),
                Vec2::new(0.5, 0.5),
            ],
            sprite: SubTexture::default(),
        }
    }
}

impl Quad {
    pub fn new_from_transform(transform: Transform) -> Self {
        let origin: [f32; 2] = [transform.position.x, transform.position.y];
        let offset: [f32; 2] = [transform.scale.x / 2.0, transform.scale.y / 2.0];
        Self {
            points: [
                Vec2::new(origin[0] - offset[0], origin[1] + offset[1]),
                Vec2::new(origin[0] - offset[0], origin[1] - offset[1]),
                Vec2::new(origin[0] + offset[0], origin[1] - offset[1]),
                Vec2::new(origin[0] + offset[0], origin[1] + offset[1]),
            ],
            ..Default::default()
        }
    }
    pub fn get_vertices(&self) -> Vec<Vertex> {
        let vertices = std::iter::zip(self.points, self.sprite.get_uv_coords());
        vertices
            .map(|(position, uv)| Vertex {
                position: [position.x, position.y],
                uv,
            })
            .collect()
    }
}

impl Renderer {
    pub fn new_with_camera(camera: Camera) -> Self {
        Self {
            camera,
            ..Default::default()
        }
    }
    pub fn new_with_camera_and_program(camera: Camera, program: Shader) -> Self {
        let gl = get_context();
        program.bind(&gl);

        Self {
            camera,
            u_time: program.get_uniform_location(&gl, "uTime"),
            u_color: program.get_uniform_location(&gl, "uColor"),
            u_model_matrix: program.get_uniform_location(&gl, "uModel"),
            u_view_matrix: program.get_uniform_location(&gl, "uView"),
            u_projection_matrix: program.get_uniform_location(&gl, "uProj"),
            program,
            gl,
            ..Default::default()
        }
    }
    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = camera;
    }
    pub fn set_shader(&mut self, program: Shader) {
        let gl = &self.gl;

        self.u_time = program.get_uniform_location(gl, "uTime");
        self.u_color = program.get_uniform_location(gl, "uColor");
        self.u_model_matrix = program.get_uniform_location(gl, "uModel");
        self.u_view_matrix = program.get_uniform_location(gl, "uView");
        self.u_projection_matrix = program.get_uniform_location(gl, "uProj");
        self.program = program;
    }
    pub fn resize(&mut self, width: f32, height: f32) {
        self.camera.set_width_and_height(width, height);
        self.gl.viewport(0, 0, width as i32, height as i32);
        self.gl.uniform_matrix4fv_with_f32_array(
            self.u_projection_matrix.as_ref(),
            false,
            self.camera.projection(),
        );
    }
    pub fn init_shader(&mut self) {
        let gl = &self.gl;
        self.program.bind(gl);

        gl.uniform1f(self.u_time.as_ref(), 0.0);
        gl.uniform4f(self.u_color.as_ref(), 1.0, 1.0, 1.0, 1.0);
        gl.uniform_matrix4fv_with_f32_array(
            self.u_view_matrix.as_ref(),
            false,
            crate::Mat4::identity().as_slice(),
        );
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
    }
    pub fn add_texture(&mut self, key: &'static str, texture: Texture) {
        self.textures.insert(key, texture);
    }
    pub fn use_texture(&mut self, key: &str) {
        let gl = &self.gl;
        if let Some(texture) = self.textures.get(key) {
            texture.bind(gl);
        } else {
            self.textures.get("MAGENTA").unwrap().bind(gl);
        }
    }
    pub fn begin_draw(&mut self) {
        let gl = &self.gl;

        self.batches.clear();

        let mesh = Mesh::new(
            gl,
            Vec::with_capacity(MAX_BATCH_VERTICES as usize),
            Vec::with_capacity(MAX_BATCH_INDICES as usize),
        );

        self.batches.push(mesh);
    }
    pub fn add_quad(&mut self, quad: Quad) {
        let gl = &self.gl;

        // Get last batch. This should never be empty becase begin_draw should have been called before.
        let mut batch = self
            .batches
            .last_mut()
            .expect("Batch list empty. Check if begin_draw was called before.");
        if batch.vertices.len() + 4 > MAX_BATCH_VERTICES as usize {
            let mesh = Mesh::new(gl, Vec::new(), Vec::new());
            self.batches.push(mesh);

            batch = self.batches.last_mut().unwrap();
        }

        let last = batch.vertices.len() as u32;
        batch.vertices.append(&mut quad.get_vertices());
        let mut indices = vec![last, last + 2, last + 1, last, last + 3, last + 2];
        batch.indices.append(&mut indices);
    }
    pub fn end_draw(&mut self) {
        let gl = &self.gl;
        gl.uniform_matrix4fv_with_f32_array(
            self.u_view_matrix.as_ref(),
            false,
            self.camera.transform.matrix(),
        );
        self.program.bind(gl);
        for (draw_call, batch) in self.batches.iter().enumerate() {
            batch.setup(gl);
            let color = draw_call as f32 / self.batches.len() as f32;
            gl.uniform4f(self.u_color.as_ref(), 0.2, color, 0.2, 1.0);
            gl.draw_elements_with_i32(
                GL::TRIANGLES,
                batch.indices.len() as i32,
                GL::UNSIGNED_INT,
                0,
            );
        }
    }
    pub fn clear(&mut self, color: [f32; 4]) {
        let gl = &self.gl;
        gl.clear_color(color[0], color[1], color[2], color[3]);
        gl.clear(GL::COLOR_BUFFER_BIT);
    }
}
