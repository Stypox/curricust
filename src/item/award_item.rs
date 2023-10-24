use resume_cv_proc_macro::{CvElementBuilder, CvRMarkdownItem, CvSectionItem};
use std::io::Write;

use crate::printers::{cv_developer_latex_printer::CvDeveloperLatexSectionItem, Printer};

#[derive(Debug, CvElementBuilder, CvRMarkdownItem, CvSectionItem)]
pub struct AwardItem {
    #[cv_element_builder(text_with_attributes)]
    pub name: String,
    #[cv_element_builder(text_with_attributes)]
    pub institution: Option<String>,
    #[cv_element_builder(text_with_attributes)]
    pub dates: String,
    #[cv_element_builder(text_with_attributes)]
    pub grade: Option<String>,
}

impl CvDeveloperLatexSectionItem for AwardItem {
    fn cvdl_print_left(&self, f: &mut Printer) -> std::io::Result<()> {
        write!(f, "{}", self.dates)
    }

    fn cvdl_print_heading(&self, f: &mut Printer) -> std::io::Result<()> {
        write!(f, "{}", self.name)?;
        if let Some(institution) = &self.institution {
            write!(f, " - {institution}")?;
        }
        Ok(())
    }

    fn cvdl_print_qualifier(&self, f: &mut Printer) -> std::io::Result<()> {
        if let Some(grade) = &self.grade {
            write!(f, "{grade}")?;
        }
        Ok(())
    }

    fn cvdl_print_description(&self, f: &mut Printer) -> std::io::Result<()> {
        Ok(())
    }
}

