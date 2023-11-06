use resume_cv_proc_macro::{CvElementBuilder, CvSectionItem};


use crate::writer::{
    latex_writer::{write_latex_command_call, LatexWriter, SectionItemLatexWriter},
    write::MyWrite,
};

#[derive(Debug, CvElementBuilder, CvSectionItem)]
pub struct ProjectItem {
    #[cv_element_builder(text_with_attributes)]
    pub name: String,
    #[cv_element_builder(text_with_attributes)]
    pub technologies: String,
    #[cv_element_builder(text_with_attributes)]
    pub links: Option<String>,
    #[cv_element_builder(text_with_attributes)]
    pub when: Option<String>,
    #[cv_element_builder(text_with_attributes)]
    pub details: Option<String>,
}

impl LatexWriter for ProjectItem {
    fn latex_write(&self, f: &mut MyWrite) -> std::io::Result<()> {
        write_latex_command_call(
            f,
            "itemproject",
            &[
                &self.name,
                &self.technologies,
                self.links.as_deref().unwrap_or(""),
                self.when.as_deref().unwrap_or(""),
                self.details.as_deref().unwrap_or(""),
            ],
        )
    }
}

impl SectionItemLatexWriter for ProjectItem {
    const SECTION_COMMAND: &'static str = "sectionproject";
}
