use web_sys::WebGl2RenderingContext as GL;
use web_sys::{WebGlTexture, HtmlImageElement};

pub fn create_texture(gl: &GL, image: &HtmlImageElement, count: u32) -> Result<WebGlTexture, String>{
    let texture = gl.create_texture().ok_or_else(|| String::from("Unable to create Texture object."))?;
    gl.active_texture(GL::TEXTURE0 + count);
    gl.bind_texture(GL::TEXTURE_2D, Some(&texture));
    gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::LINEAR as i32);
    gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::LINEAR as i32);
    gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, GL::REPEAT as i32);
    gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::REPEAT as i32);
    gl.tex_image_2d_with_u32_and_u32_and_html_image_element(GL::TEXTURE_2D, 0, 
        GL::RGBA as i32, GL::RGBA, GL::UNSIGNED_BYTE, &image)
        .expect("Failed to load texture");
    gl.generate_mipmap(GL::TEXTURE_2D);
    Ok(texture)
}