use resume_cv_proc_macro::{CvElementBuilder, CvRMarkdownItem, CvSectionItem};

#[derive(Debug, CvElementBuilder, CvRMarkdownItem, CvSectionItem)]
pub struct JobItem {
    #[cv_element_builder(text_with_attributes)]
    pub institution: String,
    #[cv_element_builder(text_with_attributes)]
    pub dates: String,
}
