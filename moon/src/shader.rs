use web_sys::WebGl2RenderingContext;
use web_sys::WebGlProgram;
use web_sys::WebGlShader;
use web_sys::WebGlUniformLocation;

#[repr(u32)]
pub enum ShaderType {
    VERTEX = WebGl2RenderingContext::VERTEX_SHADER,
    FRAGMENT = WebGl2RenderingContext::FRAGMENT_SHADER,
}

pub struct Shader {
    program: Option<WebGlProgram>,
}


impl Shader {
    /// Create a new Shader Program with default Vertex and Fragment shaders
    pub fn new(gl: &WebGl2RenderingContext) -> Self {
        let vertex_shader = Shader::create_vertex(
            gl,
            include_str!("../res/shader/default.vert.glsl"),
        )
        .expect("Could not create Vertex Shader!");

        let fragment_shader = Shader::create_fragment(
            gl,
            include_str!("../res/shader/default.frag.glsl"),
        )
        .expect("Could not create Fragment Shader!");

        let program = Shader::program_with_vertex_and_fragment(gl, &vertex_shader, &fragment_shader).ok();
        Self {
            program
        }
    }
    pub fn create_fragment(gl: &WebGl2RenderingContext, source: &str) -> Result<WebGlShader, String> {
        Self::create_with_type(gl, ShaderType::FRAGMENT, source)
    }
    pub fn create_vertex(gl: &WebGl2RenderingContext, source: &str) -> Result<WebGlShader, String> {
        Self::create_with_type(gl, ShaderType::VERTEX, source)
    }
    pub fn create_with_type(gl: &WebGl2RenderingContext, shader_type: ShaderType, source: &str) -> Result<WebGlShader, String> {
        let shader = gl
            .create_shader(shader_type as u32)
            .ok_or_else(|| String::from("Unable to create Shader object."))?;
        gl.shader_source(&shader, source);
        gl.compile_shader(&shader);

        if gl
            .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(shader)
        } else {
            Err(gl
                .get_shader_info_log(&shader)
                .unwrap_or_else(|| String::from("Could not compile shader.")))
        }
    }
    pub fn program_with_vertex_and_fragment(
        gl: &WebGl2RenderingContext,
        vertex_shader: &WebGlShader,
        fragment_shader: &WebGlShader,
    ) -> Result<WebGlProgram, String> {
        let program = gl
            .create_program()
            .ok_or_else(|| String::from("Unable to create Program object."))?;
        gl.attach_shader(&program, &vertex_shader);
        gl.attach_shader(&program, &fragment_shader);
        gl.link_program(&program);
    
        if gl
            .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            gl.delete_shader(Some(&vertex_shader));
            gl.delete_shader(Some(&fragment_shader));
            Ok(program)
        } else {
            Err(gl
                .get_program_info_log(&program)
                .unwrap_or_else(|| String::from("Could not link program.")))
        }
    }
    
    pub fn bind(&self, gl: &WebGl2RenderingContext) {
        gl.use_program(self.program.as_ref());
    }
    /// Get the location of a uniform on the `Shader`
    pub fn get_uniform_location(&self, gl: &WebGl2RenderingContext, name: &str) -> Option<WebGlUniformLocation> {
        if let program =  self.program.as_ref().unwrap() {
            gl.get_uniform_location(program, name)
        } else {
            None
        }
    }
}