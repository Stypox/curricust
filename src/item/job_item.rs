use resume_cv_proc_macro::{CvElementBuilder, CvSectionItem};
use std::io::Write;

use crate::printers::{latex_printer::LatexSectionItem, Writer, markdown_to_latex::write_markdown};

#[derive(Debug, CvElementBuilder, CvSectionItem)]
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

impl LatexSectionItem for JobItem {
    fn cvdl_print_left(&self, f: &mut Writer) -> std::io::Result<()> {
        write_markdown(f, &self.dates)?;
        if let Some(topics) = &self.topics {
            write!(f, r" \vspace{{3pt}}\linebreak \footnotesize{{")?;
            write_markdown(f, topics)?;
            write!(f, r"}}")?;
        }
        Ok(())
    }

    fn cvdl_print_heading(&self, f: &mut Writer) -> std::io::Result<()> {
        write_markdown(f, &self.title)?;
        write!(f, r"\textnormal{{ • ")?;
        write_markdown(f, &self.company)?;
        write!(f, "}}")?;
        Ok(())
    }

    fn cvdl_print_qualifier(&self, f: &mut Writer) -> std::io::Result<()> {
        write!(f, r"\textnormal{{")?;
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
