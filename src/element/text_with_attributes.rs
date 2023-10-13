use yaml_rust::Yaml;
use crate::util::yaml::YamlConversions;

#[derive(Debug)]
pub struct TextWithAttributes {
    text: String,
    attrs: Vec<String>,
}

impl TextWithAttributes {
    pub fn new(key: Yaml, value: Yaml) -> Result<(String, TextWithAttributes), String> {
        let key = key.einto_string()?;
        let value = value.einto_string()?;
        let mut key = key.split('-');

        let name = key.next().expect("split() produces at least one value").to_string();
        let attributes = key.map(|a| a.to_string()).collect();
        Ok((name, TextWithAttributes { text: value, attrs: attributes }))
    }
}