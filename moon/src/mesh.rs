use web_sys::{WebGlBuffer, WebGlVertexArrayObject};

use crate::{gl, GL};

/// The `Vertex` struct holds the data that will be later sent to WebGL in a `GL::ARRAY_BUFFER`.
/// It consists of position and color vectors, and UV co-ordinates.
#[repr(C)]
pub struct Vertex {
    pub position: [f32; 2],
    pub uv: [f32; 2],
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    vao: WebGlVertexArrayObject,
    vbo: WebGlBuffer,
    ibo: WebGlBuffer,
}

impl gl::Bind for Mesh {
    /// Bind the `WebGlVertexArrayObject` of the `Mesh`.
    fn bind(&self, gl: &GL) {
        gl.bind_vertex_array(Some(&self.vao));
    }
    fn unbind(&self, _gl: &GL) {
        _gl.bind_vertex_array(None);
    }
}

impl Mesh {
    pub fn new(gl: &GL, vertices: Vec<Vertex>, indices: Vec<u32>) -> Self {
        Self {
            vertices,
            indices,
            vao: {
                let vao = gl
                    .create_vertex_array()
                    .expect("Could not create Vertex Array Object.");
                gl.bind_vertex_array(Some(&vao));
                vao
            },
            vbo: gl.create_buffer().expect("Could not create Buffer."),
            ibo: gl.create_buffer().expect("Could not create Buffer."),
        }
    }
    /// Create a new Quad mesh with a side length of 1m
    pub fn quad(gl: &GL) -> Self {
        Self::quad_with_side(gl, 1.0)
    }
    /// Create a new Quad mesh with a given side length
    pub fn quad_with_side(gl: &GL, side: f32) -> Self {
        let half = side / 2.0;
        let vertices = vec![
            Vertex {
                position: [-half, half],
                uv: [0.0, 0.0],
            },
            Vertex {
                position: [-half, -half],
                uv: [0.0, 1.0],
            },
            Vertex {
                position: [half, -half],
                uv: [1.0, 1.0],
            },
            Vertex {
                position: [half, half],
                uv: [1.0, 0.0],
            },
        ];
        let indices: Vec<u32> = vec![0, 2, 1, 0, 3, 2];
        Self::new(gl, vertices, indices)
    }

    /// Set up the vertex (vbo) and index (ibo) `WebGlBuffer` and send their data to the GPU.
    pub fn setup(&self, gl: &GL) {
        use gl::Bind;
        self.bind(gl);

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.vbo));
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&self.ibo));

        let vertex_slice = unsafe {
            std::slice::from_raw_parts(
                self.vertices.as_ptr() as *const u8,
                self.vertices.len() * std::mem::size_of::<Vertex>(),
            )
        };
        let index_slice = unsafe {
            std::slice::from_raw_parts(
                self.indices.as_ptr() as *const u8,
                self.indices.len() * std::mem::size_of::<u32>(),
            )
        };

        gl.buffer_data_with_u8_array(GL::ARRAY_BUFFER, vertex_slice, GL::STATIC_DRAW);
        gl.buffer_data_with_u8_array(GL::ELEMENT_ARRAY_BUFFER, index_slice, GL::STATIC_DRAW);

        gl.vertex_attrib_pointer_with_i32(0, 2, GL::FLOAT, false, 4 * 4, 0);
        gl.vertex_attrib_pointer_with_i32(1, 2, GL::FLOAT, false, 4 * 4, 8);

        gl.enable_vertex_attrib_array(0 as u32);
        gl.enable_vertex_attrib_array(1 as u32);
    }
}
