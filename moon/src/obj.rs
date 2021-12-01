use std::io::BufReader;
use tobj::load_mtl_buf;
use tobj::load_obj_buf;

pub fn load_material(file: &[u8]) -> tobj::MTLLoadResult {
    let mut file = BufReader::new(file);
    load_mtl_buf(&mut file)
}

pub fn load_model(file: &[u8]) -> Vec<tobj::Model> {
    let mut file = BufReader::new(file);
    let (models, _materials) = load_obj_buf(
        &mut file,
        &tobj::LoadOptions {
            single_index: true,
            triangulate: true,
            ..Default::default()
        },
        |p| match p.file_name().unwrap().to_str().unwrap() {
            "12140_Skull_v3_L2.mtl" => {
                load_material(include_bytes!("../res/model/skull/skull.mtl"))
            }
            "matilda.mtl" => load_material(include_bytes!("../res/model/matilda/matilda.mtl")),
            _ => {
                panic!(
                    "Need to import {} material",
                    &p.file_name().unwrap().to_str().unwrap()
                )
            }
        },
    )
    .unwrap();
    models
}