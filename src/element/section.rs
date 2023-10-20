use resume_cv_proc_macro::CvElementBuilder;
use std::{fmt::Debug, io::Write};
use yaml_rust::Yaml;

use crate::{
    attr::{context::Context, text_with_attributes::TextWithAttributes},
    printers::{
        printer::Printer,
        rmarkdown::{RMarkdownPrinter, RMarkdownSectionItem},
    },
    util::yaml::YamlConversions,
};

#[derive(Debug, CvElementBuilder)]
pub struct SectionElement<T> {
    #[cv_element_builder(text_with_attributes)]
    pub title: String,
    #[cv_element_builder(text_with_attributes)]
    pub description: Option<String>,
    pub items: Option<Vec<T>>,
}

impl<T> SectionElement<T> {
    pub fn parse(hash: Yaml, ctx: &Context) -> Result<SectionElement<T>, String> {
        let hash = hash.einto_hash()?;
        let mut section = SectionElement::<T>::builder();

        for (element_type, element_value) in hash {
            let (element_type, element_value) =
                TextWithAttributes::new(element_type, element_value)?;
            match element_type.as_str() {
                "title" => section.add_title(element_value),
                "description" => section.add_description(element_value),
                _ => return Err(format!("Unknown section attribute {element_type}")),
            };
        }

        section.build(ctx)
    }
}

impl<T: RMarkdownPrinter + RMarkdownSectionItem> RMarkdownPrinter for SectionElement<T> {
    fn rmarkdown_print(&self, f: &mut Printer) -> std::io::Result<()> {
        writeln!(f, "# {}\n", self.title)?;

        if let Some(description) = &self.description {
            writeln!(f, "{description}\n")?;
        }

        if let Some(items) = &self.items {
            writeln!(f, "```{{r section}}\ntribble(")?;

            let fields = T::get_field_names();
            write!(f, "  ~ {}", fields.join(", ~ "))?;

            for item in items {
                write!(f, ",\n  ")?;
                item.rmarkdown_print(f)?;
            }
            writeln!(f, "\n)")?;
            writeln!(f, "```")?;
        }
        Ok(())
    }
}
