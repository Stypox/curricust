use resume_cv_proc_macro::{CvElementBuilder, CvRMarkdownItem, CvSectionItem};
use std::io::Write;

use crate::printers::{cv_developer_latex_printer::CvDeveloperLatexSectionItem, Printer};

#[derive(Debug, CvElementBuilder, CvRMarkdownItem, CvSectionItem)]
pub struct ProjectItem {
    #[cv_element_builder(text_with_attributes)]
    pub dates: Option<String>,
    #[cv_element_builder(text_with_attributes)]
    pub name: String,
    #[cv_element_builder(text_with_attributes)]
    pub description: String,
    #[cv_element_builder(text_with_attributes)]
    pub technologies: String,
}

impl CvDeveloperLatexSectionItem for ProjectItem {
    fn cvdl_print_left(&self, f: &mut Printer) -> std::io::Result<()> {
        if let Some(dates) = &self.dates {
            write!(f, "{}", dates)?;
        }
        Ok(())
    }

    fn cvdl_print_heading(&self, f: &mut Printer) -> std::io::Result<()> {
        write!(f, "{}", self.name)
    }

    fn cvdl_print_qualifier(&self, f: &mut Printer) -> std::io::Result<()> {
        write!(f, "{}", self.technologies) // TODO move to description and put github link here
    }

    fn cvdl_print_description(&self, f: &mut Printer) -> std::io::Result<()> {
        write!(f, "{}", self.description)
    }
}
