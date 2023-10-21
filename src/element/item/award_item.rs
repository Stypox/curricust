use resume_cv_proc_macro::{CvElementBuilder, CvRMarkdownItem, CvSectionItem};

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
