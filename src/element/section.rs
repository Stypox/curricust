use curricust_proc_macro::CvElementBuilder;
use std::{fmt::Debug, io::Write};
use yaml_rust::Yaml;

use crate::{
    attr::{context::Context, text_with_attributes::TextWithAttributes},
    writer::{latex_writer::{LatexWriter, SectionItemLatexWriter, write_latex_command_call}, write::MyWrite},
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

    // TODO subsections are quite hacky at the moment, they have not been thought through
    // e.g. it should be possible to give IDs and ordering to subsections just like items
    pub is_subsection: bool,
    pub subsections: Option<Vec<SectionElement<T>>>,
}

impl<T: SectionItem> SectionElement<T> {
    pub fn parse(ctx: &Context, is_subsection: bool, hash: Yaml) -> Result<Self, String> {
        let hash = hash.einto_hash()?;
        let mut section = SectionElement::<T>::builder();
        section.is_subsection(is_subsection);

        for (key, value) in hash {
            let key = key.einto_string()?;
            if key == "items" {
                let value = value.einto_vec()?;
                let mut items = vec![];
                for item in value {
                    if let Some(item) = T::parse(ctx, item.einto_hash()?)? {
                        // None is returned if the item is hidden
                        items.push(item);
                    }
                }
                items.sort_by(|a, b| a.0.cmp(&b.0));
                section.items(items.into_iter().map(|item| item.1).collect());

            } else if key == "subsections" {
                let value = value.einto_vec()?;
                let mut subsections = vec![];
                for item in value {
                    subsections.push(SectionElement::<T>::parse(ctx, true, item)?)
                }
                section.subsections(subsections);

            } else {
                let (key, value) = TextWithAttributes::new_string(key, value)?;
                match key.as_str() {
                    "title" => section.add_title(value),
                    "description" => section.add_description(value),
                    _ => return Err(format!("Unknown section attribute: {key}")),
                };
            }
        }

        section.build(ctx)
    }
}

impl<T: SectionItemLatexWriter> LatexWriter for SectionElement<T> {
    fn latex_write(&self, f: &mut MyWrite) -> std::io::Result<()> {
        let is_subsection_text = if self.is_subsection {
            "subsection"
        } else {
            ""
        };

        write_latex_command_call(f, T::SECTION_COMMAND, &[is_subsection_text, &self.title, self.description.as_deref().unwrap_or("")])?;
        write!(f, "{{")?;
        for item in self.items.iter().flatten() {
            item.latex_write(f)?;
        }
        writeln!(f, "}}{{")?;
        for subsection in self.subsections.iter().flatten() {
            subsection.latex_write(f)?;
        }
        writeln!(f, "}}")?;

        Ok(())
    }
}
