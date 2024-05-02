use curricust_proc_macro::{CvElementBuilder, CvSectionItem};


use crate::writer::{
    latex_writer::{write_latex_command_call, LatexWriter, SectionItemLatexWriter},
    write::MyWrite,
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

impl LatexWriter for EducationItem {
    fn latex_write(&self, f: &mut MyWrite) -> std::io::Result<()> {
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

impl SectionItemLatexWriter for EducationItem {
    const SECTION_COMMAND: &'static str = "sectioneducation";
}
