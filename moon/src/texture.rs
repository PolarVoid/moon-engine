use web_sys::WebGl2RenderingContext;
use web_sys::HtmlImageElement;
use web_sys::WebGlTexture;

pub struct Texture {
    raw: Option<WebGlTexture>,
    width: i32,
    height: i32,
}

impl Default for Texture {
    fn default() -> Self {
        Self { 
            raw: None, 
            width: 1, 
            height: 1
        }
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
    gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MIN_FILTER, WebGl2RenderingContext::LINEAR as i32);
    gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MAG_FILTER, WebGl2RenderingContext::LINEAR as i32);
    gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_S, WebGl2RenderingContext::REPEAT as i32);
    gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_T, WebGl2RenderingContext::REPEAT as i32);
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
