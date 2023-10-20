use std::{fs, path::Path};

use yaml_rust::{Yaml, YamlLoader};

use crate::attr::{context::Context, parse::try_parse_group};

use super::{error::ErrorToString, yaml::YamlConversions};

pub fn yaml_from_file(filename: &Path) -> Result<Yaml, String> {
    let file = fs::read_to_string(filename).err_str()?;
    let mut yaml_iter = YamlLoader::load_from_str(file.as_str())
        .err_str()?
        .into_iter();

    let result = yaml_iter
        .next()
        .ok_or("Empty yaml array from file".to_string())?;
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
        return Err(format!("Expected file as child, got {file_label:?}"));
    }

    yaml_from_file(&root.join(path))
}

pub fn include_file_with_context(root: &Path, ctx: Context, hash_or_string: Yaml) -> Result<(Context, Yaml), String> {
    let mut path = None;
    let mut ctx = ctx.clone();
    
    if let Yaml::Hash(hash) = hash_or_string {
        for (key, value) in hash {
            let key = key.einto_string()?;

            let Some(value) = try_parse_group(&mut ctx, &key, value)? else {
                // as attribute group was found, so we did not get back the value
                continue;
            };

            if path.is_none() && key == "file" {
                path = Some(value.einto_string()?);
            } else {
                return Err(format!("Invalid key in file inclusion {}", key));
            }
        }
    } else {
        path = Some(hash_or_string.einto_string()?)
    };

    let path = path.ok_or("Missing file: in file inclusion".to_string())?;
    Ok((ctx, yaml_from_file(&root.join(path))?))
}
