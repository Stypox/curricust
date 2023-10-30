use resume_cv_proc_macro::{CvElementBuilder, CvSectionItem};
use std::io::Write;

use crate::printers::{latex_printer::LatexSectionItem, markdown_to_latex::write_markdown, Writer};

#[derive(Debug, CvElementBuilder, CvSectionItem)]
pub struct EducationItem {
    #[cv_element_builder(text_with_attributes)]
    pub dates: String,
    #[cv_element_builder(text_with_attributes)]
    pub degree: String,
    #[cv_element_builder(text_with_attributes)]
    pub institution: String,
    #[cv_element_builder(text_with_attributes)]
    pub grade: Option<String>,
    #[cv_element_builder(text_with_attributes)]
    pub details: Option<String>,
}

impl LatexSectionItem for EducationItem {
    fn cvdl_print_left(&self, f: &mut Writer) -> std::io::Result<()> {
        write_markdown(f, &self.dates)?;
        if let Some(grade) = &self.grade {
            write!(f, r" \vspace{{3pt}}\linebreak \footnotesize{{")?;
            write_markdown(f, grade)?;
            write!(f, r"}}")?;
        }
        Ok(())
    }

    fn cvdl_print_heading(&self, f: &mut Writer) -> std::io::Result<()> {
        write_markdown(f, &self.degree)
    }

    fn cvdl_print_qualifier(&self, f: &mut Writer) -> std::io::Result<()> {
        write_markdown(f, &self.institution)
    }

    fn cvdl_print_description(&self, f: &mut Writer) -> std::io::Result<()> {
        if let Some(details) = &self.details {
            write_markdown(f, details)?;
        }
        Ok(())
    }
}
