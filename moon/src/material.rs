use std::collections::BTreeMap;
use web_sys::WebGlUniformLocation;

use crate::shader;

pub struct Material {
    shader: shader::Shader,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            shader: Default::default(),
        }
    }
}

impl Material {}
