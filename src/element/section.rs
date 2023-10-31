use resume_cv_proc_macro::CvElementBuilder;
use std::{fmt::Debug, io::Write};
use yaml_rust::Yaml;

use crate::{
    attr::{context::Context, text_with_attributes::TextWithAttributes},
    printers::{latex_printer::{LatexPrinter, SectionItemLatexPrinter, write_latex_command_call}, markdown_to_latex::write_markdown, Writer},
    util::yaml::YamlConversions,
};

use crate::item::SectionItem;

#[derive(Debug, CvElementBuilder)]
pub struct SectionElement<T> {
    #[cv_element_builder(text_with_attributes)]
    pub title: String,
    #[cv_element_builder(text_with_attributes)]
    pub description: Option<String>,
    pub items: Option<Vec<T>>,
}

impl<T: SectionItem> SectionElement<T> {
    pub fn parse(hash: Yaml, ctx: &Context) -> Result<SectionElement<T>, String> {
        let hash = hash.einto_hash()?;
        let mut section = SectionElement::<T>::builder();

        for (key, value) in hash {
            let key = key.einto_string()?;
            if key == "items" {
                let value = value.einto_vec()?;
                let mut items = vec![];
                for item in value {
                    if let Some(item) = T::parse(item.einto_hash()?, ctx)? {
                        // None is returned if the item is hidden
                        items.push(item);
                    }
                }
                items.sort_by(|a, b| a.0.cmp(&b.0));
                section.items(items.into_iter().map(|item| item.1).collect());
                continue;
            }

            let (key, value) = TextWithAttributes::new_string(key, value)?;
            match key.as_str() {
                "title" => section.add_title(value),
                "description" => section.add_description(value),
                _ => return Err(format!("Unknown section attribute: {key}")),
            };
        }

        section.build(ctx)
    }
}

#[allow(clippy::write_literal)]
impl<T: SectionItemLatexPrinter> LatexPrinter for SectionElement<T> {
    fn latex_print(&self, f: &mut Writer) -> std::io::Result<()> {
        write_latex_command_call(f, T::SECTION_COMMAND, &[&self.title, self.description.as_deref().unwrap_or("")])?;
        writeln!(f, "{{")?;
        if let Some(items) = &self.items {
            for item in items {
                item.latex_print(f)?;
            }
        }
        writeln!(f, "}}")?;
        Ok(())
    }
}
