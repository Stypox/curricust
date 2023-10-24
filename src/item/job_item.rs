use resume_cv_proc_macro::{CvElementBuilder, CvRMarkdownItem, CvSectionItem};
use std::io::Write;

use crate::printers::{cv_developer_latex_printer::CvDeveloperLatexSectionItem, Printer};

#[derive(Debug, CvElementBuilder, CvRMarkdownItem, CvSectionItem)]
pub struct JobItem {
    #[cv_element_builder(text_with_attributes)]
    pub dates: String,
    #[cv_element_builder(text_with_attributes)]
    pub institution: String,
}

impl CvDeveloperLatexSectionItem for JobItem {
    fn cvdl_print_left(&self, f: &mut Printer) -> std::io::Result<()> {
        write!(f, "{}", self.dates)
    }

    fn cvdl_print_heading(&self, f: &mut Printer) -> std::io::Result<()> {
        Ok(())
    }

    fn cvdl_print_qualifier(&self, f: &mut Printer) -> std::io::Result<()> {
        write!(f, "{}", self.institution)
    }

    fn cvdl_print_description(&self, f: &mut Printer) -> std::io::Result<()> {
        Ok(())
    }
}
