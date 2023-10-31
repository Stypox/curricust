use std::io::Write;
use std::path::Path;

use crate::attr::context::Context;
use crate::attr::parse::try_parse_group;
use crate::attr::text_with_attributes::TextWithAttributes;
use crate::printers::latex_printer::LatexPrinter;
use crate::printers::{AllPrinters, Writer};
use crate::util::file::{include_file, include_file_with_context};
use crate::util::yaml::YamlConversions;
use multimap::MultiMap;
use yaml_rust::Yaml;

use super::header::HeaderElement;
use super::section::SectionElement;
use crate::item::award_item::AwardItem;
use crate::item::education_item::EducationItem;
use crate::item::job_item::JobItem;
use crate::item::project_item::ProjectItem;
use crate::item::SectionItem;

#[derive(Debug)]
pub struct BaseElement {
    header: HeaderElement,
    sections: Vec<Box<dyn AllPrinters>>,
}

impl BaseElement {
    fn parse_dictionary(
        dictionary: &mut MultiMap<String, TextWithAttributes>,
        hash: Yaml,
    ) -> Result<(), String> {
        let hash = hash.einto_hash()?;
        for (key, value) in hash.into_iter() {
            let (key, value) = TextWithAttributes::new_yaml(key, value)?;
            dictionary.insert(key, value);
        }
        Ok(())
    }

    fn parse_section<T>(
        value: Yaml,
        sections: &mut Vec<Box<dyn AllPrinters>>,
        ctx: &Context,
    ) -> Result<(), String>
    where
        T: AllPrinters + SectionItem + 'static,
        SectionElement<T>: AllPrinters,
    {
        sections.push(Box::new(SectionElement::<T>::parse(value, ctx)?));
        Ok(())
    }

    fn parse_include_section<T: AllPrinters>(
        value: Yaml,
        sections: &mut Vec<Box<dyn AllPrinters>>,
        ctx: &Context,
        root: &Path,
    ) -> Result<(), String>
    where
        T: AllPrinters + SectionItem + 'static,
        SectionElement<T>: AllPrinters,
    {
        let (override_ctx, value) = include_file_with_context(root, ctx.clone(), value)?;
        sections.push(Box::new(SectionElement::<T>::parse(value, &override_ctx)?));
        Ok(())
    }

    pub fn new(root: &Path, array: Yaml) -> Result<BaseElement, String> {
        let array = array.einto_vec()?;
        let mut ctx = Context::default();
        let mut header = HeaderElement::builder();
        let mut sections: Vec<Box<dyn AllPrinters>> = vec![];

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
                "section-education" => {
                    Self::parse_section::<EducationItem>(value, &mut sections, &ctx)?
                }
                "include-section-education" => {
                    Self::parse_include_section::<EducationItem>(value, &mut sections, &ctx, root)?
                }
                "section-award" => Self::parse_section::<AwardItem>(value, &mut sections, &ctx)?,
                "include-section-award" => {
                    Self::parse_include_section::<AwardItem>(value, &mut sections, &ctx, root)?
                }
                "section-job" => Self::parse_section::<JobItem>(value, &mut sections, &ctx)?,
                "include-section-job" => {
                    Self::parse_include_section::<JobItem>(value, &mut sections, &ctx, root)?
                }
                "section-project" => {
                    Self::parse_section::<ProjectItem>(value, &mut sections, &ctx)?
                }
                "include-section-project" => {
                    Self::parse_include_section::<ProjectItem>(value, &mut sections, &ctx, root)?
                }
                _ => return Err(format!("Base element can't have children of type {key:?}")),
            }
        }

        let header = header.build(&ctx)?;
        Ok(BaseElement { header, sections })
    }
}

#[allow(clippy::write_literal)]
impl LatexPrinter for BaseElement {
    fn latex_print(&self, f: &mut Writer) -> std::io::Result<()> {
        writeln!(f, "{}", r#"\documentclass[11pt]{cvtemplate}"#)?;
        writeln!(f, "{}", r#"\usepackage{multicol}"#)?;
        writeln!(f, "{}", r#"\setlength{\columnsep}{0mm}"#)?;
        writeln!(f, "{}\n", r#"\begin{document}"#)?;
        self.header.latex_print(f)?;
        writeln!(f, "{{}}{{}}{{")?;
        for section in &self.sections {
            section.latex_print(f)?;
            writeln!(f)?;
        }
        writeln!(f, "}}")?;
        writeln!(f, "{}", r#"\end{document}"#)?;
        Ok(())
    }
}
