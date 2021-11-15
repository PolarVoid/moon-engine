use web_sys::WebGl2RenderingContext;
use web_sys::WebGlBuffer;
use web_sys::WebGlVertexArrayObject;

/// The `Vertex` struct
/// 
/// The `Vertex` struct holds the data that will be later sent to WebGL in a `GL::ARRAY_BUFFER`.
/// It consists of position and color vectors, and UV co-ordinates.
#[repr(C)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
    pub uv: [f32; 2],
    pub normal: [f32; 3],
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u8>,
    vao: WebGlVertexArrayObject,
    vbo: WebGlBuffer,
    ibo: WebGlBuffer,
}

pub enum Shape {
    Quad { side: f32 },
    Cube { side: f32 },
    Sphere { radius: f32 },
    Pyramid { base: f32, height: f32},
    Complex,
}

impl Mesh {
    pub fn new(gl: &WebGl2RenderingContext, vertices: Vec<Vertex>, indices: Vec<u8>) -> Self {
        Self {
            vertices,
            indices,
            vao: {
                let vao = gl.create_vertex_array().expect("Could not create Vertex Array Object.");
                gl.bind_vertex_array(Some(&vao));
                vao
            },
            vbo: {
                let vbo = gl.create_buffer().expect("Could not create Buffer.");
                gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&vbo));
                vbo
            },
            ibo: {
                let ibo = gl.create_buffer().expect("Could not create Buffer.");
                gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&ibo));
                ibo
            },
        }
    }
    pub fn primitive(gl: &WebGl2RenderingContext, shape: Shape) -> Self {
        match shape {
            _ => Self::quad(gl),
        }
    }
    fn quad(gl: &WebGl2RenderingContext) -> Self {
        let vertices = vec![
            Vertex {
                position: [-0.5, 0.0, 0.5],
                color: [0.0, 0.0, 0.0],
                uv: [0.0, 0.0],
                normal: [0.0, -1.0, 0.0],
            },
            Vertex {
                position: [-0.5, 0.0, -0.5],
                color: [0.0, 0.0, 0.0],
                uv: [0.0, 1.0],
                normal: [0.0, -1.0, 0.0],
            },
            Vertex {
                position: [0.5, 0.0, -0.5],
                color: [0.0, 0.0, 0.0],
                uv: [1.0, 1.0],
                normal: [0.0, -1.0, 0.0],
            },
            Vertex {
                position: [0.5, 0.0, 0.5],
                color: [0.0, 0.0, 0.0],
                uv: [1.0, 0.0],
                normal: [0.0, -1.0, 0.0],
            },
        ];
        let indices: Vec<u8> = vec![
            0, 1, 2,
            0, 2, 3,
        ];
        Self::new(gl, vertices, indices)
    }
}