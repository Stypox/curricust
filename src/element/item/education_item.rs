use resume_cv_proc_macro::{CvElementBuilder, CvRMarkdownItem, CvSectionItem};

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