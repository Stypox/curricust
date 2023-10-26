use resume_cv_proc_macro::{CvElementBuilder, CvRMarkdownItem, CvSectionItem};
use std::io::Write;

use crate::printers::{cv_developer_latex_printer::CvDeveloperLatexSectionItem, Writer, markdown_to_latex::write_markdown};

#[derive(Debug, CvElementBuilder, CvRMarkdownItem, CvSectionItem)]
pub struct JobItem {
    #[cv_element_builder(text_with_attributes)]
    pub dates: String,
    #[cv_element_builder(text_with_attributes)]
    pub title: String,
    #[cv_element_builder(text_with_attributes)]
    pub company: String,
    #[cv_element_builder(text_with_attributes)]
    pub where_: String,
    #[cv_element_builder(text_with_attributes)]
    pub topics: Option<String>,
    #[cv_element_builder(text_with_attributes)]
    pub details: Option<String>,
}

#[allow(clippy::write_literal)]
impl CvDeveloperLatexSectionItem for JobItem {
    fn cvdl_print_left(&self, f: &mut Writer) -> std::io::Result<()> {
        write_markdown(f, &self.dates)?;
        if let Some(topics) = &self.topics {
            write!(f, "{}", " \\hfill\\vspace{3pt}\\linebreak \\footnotesize{")?;
            write_markdown(f, topics)?;
            write!(f, "{}", r"}")?;
        }
        Ok(())
    }

    fn cvdl_print_heading(&self, f: &mut Writer) -> std::io::Result<()> {
        write_markdown(f, &self.title)
    }

    fn cvdl_print_qualifier(&self, f: &mut Writer) -> std::io::Result<()> {
        write_markdown(f, &self.company)?;
        write!(f, "\\textnormal{{ • ")?;
        write_markdown(f, &self.where_)?;
        write!(f, "}}")?;
        Ok(())
    }

    fn cvdl_print_description(&self, f: &mut Writer) -> std::io::Result<()> {
        if let Some(details) = &self.details {
            write_markdown(f, details)?;
        }
        Ok(())
    }
}
