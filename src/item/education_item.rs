use resume_cv_proc_macro::{CvElementBuilder, CvSectionItem};
use std::io::Write;

use crate::printers::{
    latex_printer::{write_latex_command_call, LatexPrinter, SectionItemLatexPrinter},
    markdown_to_latex::write_markdown,
    Writer,
};

#[derive(Debug, CvElementBuilder, CvSectionItem)]
pub struct EducationItem {
    #[cv_element_builder(text_with_attributes)]
    pub degree: String,
    #[cv_element_builder(text_with_attributes)]
    pub institution: String,
    #[cv_element_builder(text_with_attributes)]
    pub when: String,
    #[cv_element_builder(text_with_attributes)]
    pub grade: Option<String>,
    #[cv_element_builder(text_with_attributes)]
    pub details: Option<String>,
}

impl LatexPrinter for EducationItem {
    fn latex_print(&self, f: &mut Writer) -> std::io::Result<()> {
        write_latex_command_call(
            f,
            "itemeducation",
            &[
                &self.degree,
                &self.institution,
                &self.when,
                self.grade.as_deref().unwrap_or(""),
                self.details.as_deref().unwrap_or(""),
            ],
        )
    }
}

impl SectionItemLatexPrinter for EducationItem {
    const SECTION_COMMAND: &'static str = "sectioneducation";
}
