use std::io::Write;
use std::path::Path;

use crate::attr::context::{Context, AttributeType};
use crate::attr::text_with_attributes::TextWithAttributes;
use crate::printers::printer::Printer;
use crate::printers::rmarkdown::RMarkdownPrinter;
use crate::util::yaml::YamlConversions;
use crate::util::file::include_file;
use multimap::MultiMap;
use yaml_rust::Yaml;

use super::header::HeaderElement;
use super::item::education_item::EducationItem;
use super::section::SectionElement;

#[derive(Debug)]
pub struct BaseElement {
    header: HeaderElement,
    sections: Vec<Box<dyn RMarkdownPrinter>>,
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

    fn get_attrs(locale: &Option<String>, display: &Option<String>) -> Vec<String> {
        [locale.clone(), display.clone()]
            .into_iter()
            .flatten()
            .collect()
    }

    pub fn new(root: &Path, array: Yaml) -> Result<BaseElement, String> {
        let array = array.einto_vec()?;
        let mut ctx = Context::default();
        let mut header = HeaderElement::builder();
        let mut sections: Vec<Box<dyn RMarkdownPrinter>> = vec![];

        for yaml in array {
            let (element_type, element_value) = yaml.einto_single_element_hash()?;

            match element_type.as_str() {
                "locale" => ctx.set_attr(AttributeType::Locale, element_value.einto_string()?),
                "display" => ctx.set_attr(AttributeType::Display, element_value.einto_string()?),
                "dictionary" => Self::parse_dictionary(&mut ctx.dictionary, element_value)?,
                "include-dictionary" => {
                    Self::parse_dictionary(&mut ctx.dictionary, include_file(root, element_value)?)?
                }
                "header" => HeaderElement::parse(&mut header, element_value)?,
                "include-header" => {
                    HeaderElement::parse(&mut header, include_file(root, element_value)?)?
                }
                "section" => {
                    sections.push(Box::new(SectionElement::<EducationItem>::parse(element_value, &ctx)?));
                }
                _ => {} //return Err(format!("Base element can't have children of type {element_type:?}")),
            }
        }

        let header = header.build(&ctx)?;
        Ok(BaseElement {
            header,
            sections,
        })
    }
}

impl RMarkdownPrinter for BaseElement {
    fn rmarkdown_print(&self, f: &mut Printer) -> std::io::Result<()> {
        writeln!(f, "---")?;
        self.header.rmarkdown_print(f)?;
        writeln!(f, "---\n")?;

        for section in &self.sections {
            section.rmarkdown_print(f)?;
        }
        Ok(())
    }
}
