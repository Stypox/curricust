use resume_cv_proc_macro::{CvElementBuilder, CvRMarkdownItem, CvSectionItem};

#[derive(Debug, CvElementBuilder, CvRMarkdownItem, CvSectionItem)]
pub struct ProjectItem {
    #[cv_element_builder(text_with_attributes)]
    pub name: String,
    #[cv_element_builder(text_with_attributes)]
    pub description: String,
    #[cv_element_builder(text_with_attributes)]
    pub technologies: String,
}
