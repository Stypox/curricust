use std::{path::Path, fs};

use yaml_rust::{Yaml, YamlLoader};

use super::{error::ErrorToString, yaml::YamlConversions};


pub fn yaml_from_file(filename: &Path) -> Result<Yaml, String> {
    let file = fs::read_to_string(filename).err_str()?;
    let mut yaml_iter = YamlLoader::load_from_str(file.as_str()).err_str()?.into_iter();

    let result = yaml_iter.next().ok_or("Empty yaml array from file".to_string())?;
    if let Some(e) = yaml_iter.next() {
        Err(format!("More than one yaml from file: {e:?}"))
    } else {
        Ok(result)
    }
}

pub fn include_file(root: &Path, hash: Yaml) -> Result<Yaml, String> {
    let (file_label, path) = hash.einto_single_element_hash()?;
    let path = path.einto_string()?;

    if file_label != "file" {
        return Err(format!("Expected file as child, got {file_label:?}"))
    }

    yaml_from_file(&root.join(path))
}