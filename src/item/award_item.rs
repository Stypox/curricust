use resume_cv_proc_macro::{CvElementBuilder, CvSectionItem};
use std::io::Write;

use crate::printers::{cv_developer_latex_printer::CvDeveloperLatexSectionItem, Writer, markdown_to_latex::write_markdown};

#[derive(Debug, CvElementBuilder, CvSectionItem)]
pub struct AwardItem {
    #[cv_element_builder(text_with_attributes)]
    pub dates: String,
    #[cv_element_builder(text_with_attributes)]
    pub name: String,
    #[cv_element_builder(text_with_attributes)]
    pub institution: Option<String>,
    #[cv_element_builder(text_with_attributes)]
    pub grade: Option<String>,
}

impl CvDeveloperLatexSectionItem for AwardItem {
    fn cvdl_print_left(&self, f: &mut Writer) -> std::io::Result<()> {
        write_markdown(f, &self.dates)
    }

    fn cvdl_print_heading(&self, f: &mut Writer) -> std::io::Result<()> {
        write_markdown(f, &self.name)?;
        if let Some(institution) = &self.institution {
            write!(f, " - ")?;
            write_markdown(f, institution)?;
        }
        Ok(())
    }

    fn cvdl_print_qualifier(&self, f: &mut Writer) -> std::io::Result<()> {
        if let Some(grade) = &self.grade {
            write_markdown(f, grade)?;
        }
        Ok(())
    }

    fn cvdl_print_description(&self, _f: &mut Writer) -> std::io::Result<()> {
        Ok(())
    }
}
