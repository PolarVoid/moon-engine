use nalgebra::Vector3;

pub trait Transform {
    fn translate(&mut self, shift: &Vector3<f32>);
}