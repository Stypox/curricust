use yaml_rust::yaml::Hash;
use yaml_rust::Yaml;

pub trait YamlConversions {
    fn einto_string(self) -> Result<String, String>;
    fn einto_nullable_string(self) -> Result<Option<String>, String>;
    fn einto_hash(self) -> Result<Hash, String>;
    fn einto_vec(self) -> Result<Vec<Yaml>, String>;
    fn einto_single_element_hash(self) -> Result<(String, Yaml), String>;
    fn einto_nullable_int(self) -> Result<Option<i64>, String>;
}

impl YamlConversions for Yaml {
    fn einto_string(self) -> Result<String, String> {
        match self {
            Yaml::Real(a) => Ok(a),
            Yaml::Integer(a) => Ok(a.to_string()),
            Yaml::String(a) => Ok(a),
            Yaml::Boolean(a) => Ok(a.to_string()),
            Yaml::Array(a) => Err(format!("Expected string, got array: {a:?}")),
            Yaml::Hash(a) => Err(format!("Expected string, got map: {a:?}")),
            Yaml::Alias(a) => Err(format!("Expected string, got alias: {a:?}")),
            Yaml::Null => Err("Expected string, got null".to_string()),
            Yaml::BadValue => Err("Expected string, got bad value".to_string()),
        }
    }

    fn einto_nullable_string(self) -> Result<Option<String>, String> {
        if let Yaml::Null = self {
            Ok(None)
        } else {
            Ok(Some(self.einto_string()?))
        }
    }

    fn einto_hash(self) -> Result<Hash, String> {
        self.into_hash().ok_or("Expected hash".to_string())
    }

    fn einto_vec(self) -> Result<Vec<Yaml>, String> {
        self.into_vec().ok_or("Expected array".to_string())
    }

    fn einto_single_element_hash(self) -> Result<(String, Yaml), String> {
        let hash = self.einto_hash()?;
        let mut hash_iter = hash.into_iter();

        let (element_type, element_value) =
            hash_iter.next().ok_or("Unexpected empty map".to_string())?;
        let element_type = element_type.einto_string()?;

        if let Some(v) = hash_iter.next() {
            Err(format!(
                "Unexpected sibiling of element type {element_type:?}: {v:?}"
            ))
        } else {
            Ok((element_type, element_value))
        }
    }

    fn einto_nullable_int(self) -> Result<Option<i64>, String> {
        if let Yaml::Null = self {
            Ok(None)
        } else if let Yaml::Integer(v) = self {
            Ok(Some(v))
        } else {
            Err("Missing".to_string())
        }
    }
}
