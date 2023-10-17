use std::io::Write;
use std::path::Path;

use crate::printers::rmarkdown::RMarkdownPrinter;
use crate::util::yaml::YamlConversions;
use crate::{
    element::text_with_attributes::TextWithAttributes,
    util::{error::ErrorToString, file::include_file},
};
use multimap::MultiMap;
use yaml_rust::Yaml;

use super::header::{HeaderElement, HeaderElementBuilder};

#[derive(Debug)]
pub struct BaseElement {
    dictionary: MultiMap<String, TextWithAttributes>,
    header: HeaderElement,
}

impl BaseElement {
    fn parse_dictionary(
        dictionary: &mut MultiMap<String, TextWithAttributes>,
        hash: Yaml,
    ) -> Result<(), String> {
        let hash = hash.einto_hash()?;
        for (key, value) in hash.into_iter() {
            let (key, value) = TextWithAttributes::new(key, value)?;
            dictionary.insert(key, value);
        }
        Ok(())
    }

    fn get_attrs(locale: Option<String>, display: Option<String>) -> Vec<String> {
        [locale.clone(), display.clone()].into_iter().filter_map(|e| e).collect()
    }

    pub fn new(root: &Path, array: Yaml) -> Result<BaseElement, String> {
        let array = array.einto_vec()?;
        let mut locale = None;
        let mut display = None;
        let mut dictionary = MultiMap::new();
        let mut header = HeaderElementBuilder::default();

        for yaml in array {
            let (element_type, element_value) = yaml.einto_single_element_hash()?;

            match element_type.as_str() {
                "locale" => locale = Some(element_value.einto_string()?),
                "display" => display = Some(element_value.einto_string()?),
                "dictionary" => Self::parse_dictionary(&mut dictionary, element_value)?,
                "include-dictionary" => {
                    Self::parse_dictionary(&mut dictionary, include_file(root, element_value)?)?
                }
                "header" => HeaderElement::parse(&mut header, element_value)?,
                "include-header" => {
                    HeaderElement::parse(&mut header, include_file(root, element_value)?)?
                }
                _ => {} //return Err(format!("Base element can't have children of type {element_type:?}")),
            }
        }

        let header = header.build(&Self::get_attrs(locale, display))?;
        Ok(BaseElement {
            dictionary,
            header,
        })
    }
}

impl<T: Write> RMarkdownPrinter<T> for BaseElement {
    fn rmarkdown_print(&self, f: &mut T) -> std::io::Result<()> {
        self.header.rmarkdown_print(f)
    }
}
