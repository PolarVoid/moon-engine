use crate::Transform;
use crate::Mesh;

pub struct Object {
    pub transform: Transform,
    mesh: Option<Mesh>,
    children: Vec<Object>,
}

impl Object {
    pub fn new() -> Self {
        Self {
            transform: Transform::new(),
            mesh: None,
            children: Vec::new(),
        }
    }
    pub fn new_with_transform(transform: Transform) -> Self {
        Self {
            transform,
            ..Self::new()
        }
    }
    pub fn new_with_mesh(mesh: Option<Mesh>) -> Self {
        Self {
            mesh,
            ..Self::new()
        }
    }
    pub fn new_with_transform_and_mesh(transform: Transform, mesh: Option<Mesh>) -> Self {
        Self {
            transform,
            mesh,
            ..Self::new()
        }
    }
    pub fn add_child(&mut self, child: Object) {
        self.children.push(child);
    }
}