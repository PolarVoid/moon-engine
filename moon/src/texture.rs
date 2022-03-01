use web_sys::HtmlImageElement;
use web_sys::WebGlTexture;

use crate::{gl, GL};

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
}
