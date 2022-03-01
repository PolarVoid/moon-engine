use web_sys::HtmlImageElement;
use web_sys::WebGl2RenderingContext;
use web_sys::WebGlTexture;

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

impl Texture {
    pub fn new(gl: &WebGl2RenderingContext, image: &HtmlImageElement) -> Self {
        let (width, height) = (image.width(), image.height());

        let texture = gl.create_texture();
        gl.active_texture(WebGl2RenderingContext::TEXTURE0);
        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, texture.as_ref());
        gl.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_MIN_FILTER,
            WebGl2RenderingContext::LINEAR as i32,
        );
        gl.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_MAG_FILTER,
            WebGl2RenderingContext::LINEAR as i32,
        );
        gl.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_WRAP_S,
            WebGl2RenderingContext::REPEAT as i32,
        );
        gl.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_WRAP_T,
            WebGl2RenderingContext::REPEAT as i32,
        );
        // Flip the Y-axis so the image displays the right way up
        gl.pixel_storei(WebGl2RenderingContext::UNPACK_FLIP_Y_WEBGL, 1);
        gl.tex_image_2d_with_u32_and_u32_and_html_image_element(
            WebGl2RenderingContext::TEXTURE_2D,
            0,
            WebGl2RenderingContext::RGBA as i32,
            WebGl2RenderingContext::RGBA,
            WebGl2RenderingContext::UNSIGNED_BYTE,
            &image,
        )
        .expect("Failed to load texture");
        gl.generate_mipmap(WebGl2RenderingContext::TEXTURE_2D);

        Self {
            width,
            height,
            texture,
            _slot: 0
        }
    }
    pub fn bind(&self, gl: &WebGl2RenderingContext) {
        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, self.texture.as_ref());
    } 
}

pub fn create_texture(
    gl: &WebGl2RenderingContext,
    image: &HtmlImageElement,
    count: u32,
) -> Result<WebGlTexture, String> {
    let texture = gl
        .create_texture()
        .ok_or_else(|| String::from("Unable to create Texture object."))?;
    gl.active_texture(WebGl2RenderingContext::TEXTURE0 + count);
    gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));
    gl.tex_parameteri(
        WebGl2RenderingContext::TEXTURE_2D,
        WebGl2RenderingContext::TEXTURE_MIN_FILTER,
        WebGl2RenderingContext::LINEAR as i32,
    );
    gl.tex_parameteri(
        WebGl2RenderingContext::TEXTURE_2D,
        WebGl2RenderingContext::TEXTURE_MAG_FILTER,
        WebGl2RenderingContext::LINEAR as i32,
    );
    gl.tex_parameteri(
        WebGl2RenderingContext::TEXTURE_2D,
        WebGl2RenderingContext::TEXTURE_WRAP_S,
        WebGl2RenderingContext::REPEAT as i32,
    );
    gl.tex_parameteri(
        WebGl2RenderingContext::TEXTURE_2D,
        WebGl2RenderingContext::TEXTURE_WRAP_T,
        WebGl2RenderingContext::REPEAT as i32,
    );
    gl.pixel_storei(WebGl2RenderingContext::UNPACK_FLIP_Y_WEBGL, 1);
    gl.tex_image_2d_with_u32_and_u32_and_html_image_element(
        WebGl2RenderingContext::TEXTURE_2D,
        0,
        WebGl2RenderingContext::RGBA as i32,
        WebGl2RenderingContext::RGBA,
        WebGl2RenderingContext::UNSIGNED_BYTE,
        &image,
    )
    .expect("Failed to load texture");
    gl.generate_mipmap(WebGl2RenderingContext::TEXTURE_2D);
    Ok(texture)
}
