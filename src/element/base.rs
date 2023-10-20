use std::io::Write;
use std::path::Path;

use crate::attr::context::Context;
use crate::attr::parse::try_parse_group;
use crate::attr::text_with_attributes::TextWithAttributes;
use crate::printers::printer::Printer;
use crate::printers::rmarkdown::RMarkdownPrinter;
use crate::util::file::{include_file, include_file_with_context};
use crate::util::yaml::YamlConversions;
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

    fn parse_section<T>(
        value: Yaml,
        sections: &mut Vec<Box<dyn RMarkdownPrinter>>,
        ctx: &Context,
    ) -> Result<(), String>
    where
        T: RMarkdownPrinter + 'static,
        SectionElement<T>: RMarkdownPrinter,
    {
        sections.push(Box::new(SectionElement::<T>::parse(value, &ctx)?));
        Ok(())
    }

    fn parse_include_section<T: RMarkdownPrinter>(
        value: Yaml,
        sections: &mut Vec<Box<dyn RMarkdownPrinter>>,
        ctx: &Context,
        root: &Path,
    ) -> Result<(), String>
    where
        T: RMarkdownPrinter + 'static,
        SectionElement<T>: RMarkdownPrinter,
    {
        let (override_ctx, value) = include_file_with_context(root, ctx.clone(), value)?;
        sections.push(Box::new(SectionElement::<T>::parse(value, &override_ctx)?));
        Ok(())
    }

    pub fn new(root: &Path, array: Yaml) -> Result<BaseElement, String> {
        let array = array.einto_vec()?;
        let mut ctx = Context::default();
        let mut header = HeaderElement::builder();
        let mut sections: Vec<Box<dyn RMarkdownPrinter>> = vec![];

        for yaml in array {
            let (key, value) = yaml.einto_single_element_hash()?;

            let Some(value) = try_parse_group(&mut ctx, &key, value)? else {
                continue;
            };

            match key.as_str() {
                "dictionary" => Self::parse_dictionary(&mut ctx.dictionary, value)?,
                "include-dictionary" => {
                    Self::parse_dictionary(&mut ctx.dictionary, include_file(root, value)?)?
                }
                "header" => HeaderElement::parse(&mut header, value)?,
                "include-header" => HeaderElement::parse(&mut header, include_file(root, value)?)?,
                "section" => Self::parse_section::<EducationItem>(value, &mut sections, &ctx)?,
                "include-section" => {
                    Self::parse_include_section::<EducationItem>(value, &mut sections, &ctx, &root)?
                }
                _ => {} //return Err(format!("Base element can't have children of type {element_type:?}")),
            }
        }

        let header = header.build(&ctx)?;
        Ok(BaseElement { header, sections })
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
