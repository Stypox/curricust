use resume_cv_proc_macro::{CvElementBuilder, CvRMarkdownItem, CvSectionItem};
use std::io::Write;

use crate::printers::{cv_developer_latex_printer::CvDeveloperLatexSectionItem, Printer};

#[derive(Debug, CvElementBuilder, CvRMarkdownItem, CvSectionItem)]
pub struct EducationItem {
    #[cv_element_builder(text_with_attributes)]
    pub degree: String,
    #[cv_element_builder(text_with_attributes)]
    pub institution: String,
    #[cv_element_builder(text_with_attributes)]
    pub dates: String,
    #[cv_element_builder(text_with_attributes)]
    pub grade: Option<String>,
    #[cv_element_builder(text_with_attributes)]
    pub details: Option<String>,
}

impl CvDeveloperLatexSectionItem for EducationItem {
    fn cvdl_print_left(&self, f: &mut Printer) -> std::io::Result<()> {
        write!(f, "{}", self.dates)
    }

    fn cvdl_print_heading(&self, f: &mut Printer) -> std::io::Result<()> {
        write!(f, "{}", self.degree)
    }

    fn cvdl_print_qualifier(&self, f: &mut Printer) -> std::io::Result<()> {
        write!(f, "{}", self.institution)
    }

    fn cvdl_print_description(&self, f: &mut Printer) -> std::io::Result<()> {
        if let Some(details) = &self.details {
            write!(f, "{details}")?;
        }
        Ok(())
    }
}
