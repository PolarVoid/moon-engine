use wasm_bindgen::JsCast;
use web_sys::HtmlImageElement;
use web_sys::WebGlTexture;

use crate::{gl, GL};

#[derive(Debug)]
pub struct Texture {
    texture: Option<WebGlTexture>,
    pub width: u32,
    pub height: u32,
    _slot: i32,
}

impl Default for Texture {
    fn default() -> Self {
        Self {
            texture: None,
            width: 1,
            height: 1,
            _slot: 0,
        }
    }
}

impl gl::Bind for Texture {
    fn bind(&self, gl: &GL) {
        gl.bind_texture(GL::TEXTURE_2D, self.texture.as_ref());
    }
    fn unbind(&self, _gl: &GL) {
        _gl.bind_texture(GL::TEXTURE_2D, None);
    }
}

impl Texture {
    pub fn new(gl: &GL, image: &HtmlImageElement) -> Self {
        let (width, height) = (image.width(), image.height());

        let texture = gl.create_texture();
        gl.active_texture(GL::TEXTURE0);
        gl.bind_texture(GL::TEXTURE_2D, texture.as_ref());
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::LINEAR as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::LINEAR as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, GL::REPEAT as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::REPEAT as i32);
        // Flip the Y-axis so the image displays the right way up
        gl.pixel_storei(GL::UNPACK_FLIP_Y_WEBGL, 1);
        gl.tex_image_2d_with_u32_and_u32_and_html_image_element(
            GL::TEXTURE_2D,
            0,
            GL::RGBA as i32,
            GL::RGBA,
            GL::UNSIGNED_BYTE,
            &image,
        )
        .expect("Failed to load texture");
        gl.generate_mipmap(GL::TEXTURE_2D);

        Self {
            width,
            height,
            texture,
            _slot: 0,
        }
    }
    pub fn new_with_element_id(gl: &GL, image_src: &str) -> Self {
        let document: web_sys::Document = web_sys::window().unwrap().document().unwrap();
        let image = document
            .get_element_by_id(image_src)
            .unwrap()
            .dyn_into::<web_sys::HtmlImageElement>()
            .unwrap();
        Self::new(gl, &image)
    }
    
    /// Create a new `Texture` from an `HtmlImageElement` with an element ID in the format **textureXX** where *XX* is a number
    pub fn new_with_texture_id(gl: &GL, count: u32) -> Self {
        Self::new_with_element_id(gl, &format!("texture{}", count))
    }
}

#[derive(Debug)]
pub struct SubTexture {
    texture: Option<Box<Texture>>,
    min: [f32; 2],
    max: [f32; 2],
}

impl Default for SubTexture {
    fn default() -> Self {
        Self { 
            texture: None,
            min: [0.0, 0.0],
            max: [1.0, 1.0]
        }
    }
}

impl SubTexture {
    pub fn new(texture: Box<Texture>) -> Self {
        Self {
            texture: Some(texture),
            ..Default::default()
        }
    }
    pub fn new_with_coords(texture: Box<Texture>, uv: [f32; 4]) -> Self {
        Self {
            texture: Some(texture),
            min: [uv[0], uv[2]],
            max: [uv[1], uv[3]],
        }
    }
    pub fn get_uv_coords(&self) -> [[f32; 2]; 4] {
        let (min, max) = (self.min, self.max);
        [
            [min[0], min[1]],
            [min[0], max[1]],
            [max[0], max[1]],
            [max[0], min[1]],
        ]
    }
}

impl gl::Bind for SubTexture {
    fn bind(&self, gl: &GL) {
        if let Some(texture) = &self.texture {
            texture.bind(gl);
        }
    }
    fn unbind(&self, _gl: &GL) {
        _gl.bind_texture(GL::TEXTURE_2D, None);
    }
}