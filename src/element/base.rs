
use std::path::Path;

use crate::{element::text_with_attributes::TextWithAttributes, util::file::include_file};
use multimap::MultiMap;
use crate::util::yaml::YamlConversions;
use yaml_rust::Yaml;

#[derive(Debug)]
pub struct BaseElement {
    locale: String,
    dictionary: MultiMap<String, TextWithAttributes>,
}

impl BaseElement {
    fn parse_dictionary(dictionary: &mut MultiMap<String, TextWithAttributes>, hash: Yaml) -> Result<(), String> {
        let hash = hash.einto_hash()?;
        for (key, value) in hash.into_iter() {
            let (key, value) = TextWithAttributes::new(key, value)?;
            dictionary.insert(key, value);
        }
        Ok(())
    }

    pub fn new(root: &Path, array: Yaml) -> Result<BaseElement, String> {
        let array = array.einto_vec()?;
        let mut locale = None;
        let mut dictionary = MultiMap::new();

        for yaml in array {
            let (element_type, element_value) = yaml.einto_single_element_hash()?;

            match element_type.as_str() {
                "locale" => locale = Some(element_value.einto_string()?),
                "dictionary" => Self::parse_dictionary(&mut dictionary, element_value)?,
                "include-dictionary" => Self::parse_dictionary(&mut dictionary, include_file(root, element_value)?)?,
                _ => {}//return Err(format!("Base element can't have children of type {element_type:?}")),
            }
        }

        let locale = locale.ok_or("Did not find locale in base element")?;
        Ok(BaseElement { locale, dictionary })
    }
}