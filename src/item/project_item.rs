use resume_cv_proc_macro::{CvElementBuilder, CvRMarkdownItem, CvSectionItem};
use std::io::Write;

use crate::printers::{cv_developer_latex_printer::CvDeveloperLatexSectionItem, Writer, markdown_to_latex::write_markdown};

#[derive(Debug, CvElementBuilder, CvRMarkdownItem, CvSectionItem)]
pub struct ProjectItem {
    #[cv_element_builder(text_with_attributes)]
    pub dates: Option<String>,
    #[cv_element_builder(text_with_attributes)]
    pub name: String,
    #[cv_element_builder(text_with_attributes)]
    pub technologies: String,
    #[cv_element_builder(text_with_attributes)]
    pub links: Option<String>,
    #[cv_element_builder(text_with_attributes)]
    pub details: Option<String>,
}

impl CvDeveloperLatexSectionItem for ProjectItem {
    fn cvdl_print_left(&self, f: &mut Writer) -> std::io::Result<()> {
        if let Some(dates) = &self.dates {
            write_markdown(f, dates)?;
            write!(f, r" \vspace{{3pt}}\linebreak ")?;
        }
        write!(f, r"\footnotesize{{")?;
        write_markdown(f, &self.technologies)?;
        write!(f, r"}}")?;
        Ok(())
    }

    fn cvdl_print_heading(&self, f: &mut Writer) -> std::io::Result<()> {
        write_markdown(f, &self.name)
    }

    fn cvdl_print_qualifier(&self, f: &mut Writer) -> std::io::Result<()> {
        if let Some(links) = &self.links {
            write_markdown(f, links)?;
        }
        Ok(())    }

    fn cvdl_print_description(&self, f: &mut Writer) -> std::io::Result<()> {
        if let Some(details) = &self.details {
            write_markdown(f, details)?;
        }
        Ok(())
    }
}
