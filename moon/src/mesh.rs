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
pub enum Shape {
    Quad(f32),
    Cube(f32),
    Sphere(f32),
    Pyramid(f32, f32),
    Complex,
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u8>,
    vao: WebGlVertexArrayObject,
    vbo: WebGlBuffer,
    ibo: WebGlBuffer,
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
            vbo: gl.create_buffer().expect("Could not create Buffer."),
            ibo: gl.create_buffer().expect("Could not create Buffer."),
        }
    }
    pub fn primitive(gl: &WebGl2RenderingContext, shape: Shape) -> Self {
        match shape {
            Shape::Quad(side) => Self::quad_with_side(gl, side),
            _ => Self::quad(gl),
        }
    }
    /// Create a new Quad mesh with a side length of 1m
    pub fn quad(gl: &WebGl2RenderingContext) -> Self {
        Self::quad_with_side(gl, 1.0)
    }
    /// Create a new Quad mesh with a given side length
    pub fn quad_with_side(gl: &WebGl2RenderingContext, side: f32) -> Self {
        let half = side/2.0;
        let vertices = vec![
            Vertex {
                position: [-half, 0.0, half],
                color: [0.0, 0.0, 0.0],
                uv: [0.0, 0.0],
                normal: [0.0, -1.0, 0.0],
            },
            Vertex {
                position: [-half, 0.0, -half],
                color: [0.0, 0.0, 0.0],
                uv: [0.0, 1.0],
                normal: [0.0, -1.0, 0.0],
            },
            Vertex {
                position: [half, 0.0, -half],
                color: [0.0, 0.0, 0.0],
                uv: [1.0, 1.0],
                normal: [0.0, -1.0, 0.0],
            },
            Vertex {
                position: [half, 0.0, half],
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
    /// Set up the vertex (vbo) and index (ibo) `WebGlBuffer` and send their data to the GPU.
    pub fn setup(&self, gl: &WebGl2RenderingContext) {
        self.bind(gl);
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&self.vbo));
        gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&self.ibo));

        let u8_slice = unsafe {
            std::slice::from_raw_parts(
                self.vertices.as_ptr() as *const u8, self.vertices.len()*std::mem::size_of::<Vertex>()
            )
        };
        gl.buffer_data_with_u8_array(WebGl2RenderingContext::ARRAY_BUFFER, u8_slice, WebGl2RenderingContext::STATIC_DRAW);
        gl.buffer_data_with_u8_array(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, &self.indices, WebGl2RenderingContext::STATIC_DRAW);
    }
    /// Bind the `WebGlVertexArrayObject` of the `Mesh`.
    pub fn bind(&self, gl: &WebGl2RenderingContext) {
        gl.bind_vertex_array(Some(&self.vao));
    }
}