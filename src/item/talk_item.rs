use resume_cv_proc_macro::{CvElementBuilder, CvSectionItem};


use crate::writer::{
    latex_writer::{write_latex_command_call, LatexWriter, SectionItemLatexWriter},
    MyWrite,
};

#[derive(Debug, CvElementBuilder, CvSectionItem)]
pub struct TalkItem {
    #[cv_element_builder(text_with_attributes)]
    pub name: String,
    #[cv_element_builder(text_with_attributes)]
    pub event: String,
    #[cv_element_builder(text_with_attributes)]
    pub when: String,
}

impl LatexWriter for TalkItem {
    fn latex_write(&self, f: &mut MyWrite) -> std::io::Result<()> {
        write_latex_command_call(
            f,
            "itemtalk",
            &[&self.name, &self.event, &self.when],
        )
    }
}

impl SectionItemLatexWriter for TalkItem {
    const SECTION_COMMAND: &'static str = "sectiontalk";
}
