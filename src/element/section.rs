use resume_cv_proc_macro::CvElementBuilder;
use std::{fmt::Debug, io::Write};
use yaml_rust::Yaml;

use crate::{
    attr::{context::Context, text_with_attributes::TextWithAttributes},
    printers::{
        Printer,
        rmarkdown::{RMarkdownPrinter, RMarkdownSectionItem}, cv_developer_latex_printer::CvDeveloperLatexPrinter,
    },
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
                    items.push(T::parse(item.einto_hash()?, ctx)?);
                }
                items.sort_by(|a,b| { a.0.cmp(&b.0) });
                section.items(items.into_iter().map(|item| item.1).collect());
                continue;
            }

            let (key, value) = TextWithAttributes::new_string(key, value)?;
            match key.as_str() {
                "title" => section.add_title(value),
                "description" => section.add_description(value),
                _ => return Err(format!("Unknown section attribute {key}")),
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
            write!(f, "detailed_entries(")?;
            write!(f, "{}", fields.join(", "))?;
            writeln!(f, ")")?;
            writeln!(f, "```\n")?;
        }
        Ok(())
    }
}

#[allow(clippy::write_literal)]
impl<T: CvDeveloperLatexPrinter> CvDeveloperLatexPrinter for SectionElement<T> {
    fn cvdl_print(&self, f: &mut Printer) -> std::io::Result<()> {
        writeln!(f, "{}{}{}", r#"\cvsect{"#, self.title, r#"}"#)?;
        // TODO self.description
        if let Some(items) = &self.items {
            writeln!(f, "{}", r#"\begin{entrylist}"#)?;
            for item in items {
                item.cvdl_print(f)?;
            }
            writeln!(f, "{}", r#"\end{entrylist}"#)?;
        }
        Ok(())
    }
}