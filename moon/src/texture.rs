use std::rc::Rc;

use wasm_bindgen::JsCast;
use web_sys::HtmlImageElement;
use web_sys::WebGlTexture;

use crate::gl::Bind;
use crate::Color32;
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

impl Bind for Texture {
    fn bind(&self, gl: &GL) {
        gl.bind_texture(GL::TEXTURE_2D, self.texture.as_ref());
    }
    fn unbind(&self, _gl: &GL) {
        _gl.bind_texture(GL::TEXTURE_2D, None);
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        let gl = gl::get_context();

        gl.delete_texture(self.texture.as_ref());
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
            image,
        )
        .expect("Failed to load texture");
        gl.generate_mipmap(GL::TEXTURE_2D);

        Self {
            width,
            height,
            texture,
            ..Default::default()
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
    pub fn new_from_pixels(gl: &GL, width: u32, height: u32, pixels: &[u8]) -> Self {
        assert!(pixels.len() == (width * height * 4) as usize);
        let texture = gl.create_texture();
        gl.active_texture(GL::TEXTURE0);
        gl.bind_texture(GL::TEXTURE_2D, texture.as_ref());
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::NEAREST as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::NEAREST as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, GL::REPEAT as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::REPEAT as i32);
        gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
            GL::TEXTURE_2D,
            0,
            GL::RGBA as i32,
            width as i32,
            height as i32,
            0,
            GL::RGBA,
            GL::UNSIGNED_BYTE,
            Some(pixels),
        )
        .expect("Failed to generate texture");
        gl.generate_mipmap(GL::TEXTURE_2D);
        Self {
            width,
            height,
            texture,
            ..Default::default()
        }
    }
    pub fn colored(gl: &GL, color: Color32) -> Self {
        Self::new_from_pixels(gl, 1, 1, &<[u8; 4]>::from(color))
    }
    pub fn white(gl: &GL) -> Self {
        Self::colored(gl, crate::WHITE)
    }
    pub fn checkerboard(gl: &GL) -> Self {
        Self::checkerboard_colored(gl, crate::WHITE, crate::BLACK)
    }
    pub fn checkerboard_colored(gl: &GL, color1: Color32, color2: Color32) -> Self {
        let size = 8;
        let mut pixels = Vec::<u8>::with_capacity(size * size);
        for x_offset in 0..size {
            for y_offset in 0..size {
                let color: Color32;
                if (x_offset + y_offset) % 2 == 0 {
                    color = color1;
                } else {
                    color = color2;
                }
                pixels.append(&mut Vec::from(color));
            }
        }
        Self::new_from_pixels(gl, size as u32, size as u32, &pixels)
    }
}

#[derive(Debug)]
pub struct SubTexture {
    texture: Option<Rc<Texture>>,
    min: [f32; 2],
    max: [f32; 2],
}

impl Default for SubTexture {
    fn default() -> Self {
        Self {
            texture: None,
            min: [0.0, 0.0],
            max: [1.0, 1.0],
        }
    }
}

impl SubTexture {
    pub fn new(texture: Rc<Texture>) -> Self {
        Self {
            texture: Some(texture),
            ..Default::default()
        }
    }
    pub fn new_with_coords(texture: Rc<Texture>, uv: Color32) -> Self {
        Self {
            texture: Some(texture),
            min: [uv.x(), uv.z()],
            max: [uv.y(), uv.w()],
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

impl Bind for SubTexture {
    fn bind(&self, gl: &GL) {
        if let Some(texture) = &self.texture {
            texture.bind(gl);
        }
    }
    fn unbind(&self, _gl: &GL) {
        _gl.bind_texture(GL::TEXTURE_2D, None);
    }
}
