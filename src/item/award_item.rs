use resume_cv_proc_macro::{CvElementBuilder, CvSectionItem};


use crate::printers::{
    latex_printer::{write_latex_command_call, LatexPrinter, SectionItemLatexPrinter},
    Writer,
};

#[derive(Debug, CvElementBuilder, CvSectionItem)]
pub struct AwardItem {
    #[cv_element_builder(text_with_attributes)]
    pub name: String,
    #[cv_element_builder(text_with_attributes)]
    pub when: String,
    #[cv_element_builder(text_with_attributes)]
    pub grade: Option<String>,
}

impl LatexPrinter for AwardItem {
    fn latex_print(&self, f: &mut Writer) -> std::io::Result<()> {
        write_latex_command_call(
            f,
            "itemaward",
            &[&self.name, &self.when, self.grade.as_deref().unwrap_or("")],
        )
    }
}

impl SectionItemLatexPrinter for AwardItem {
    const SECTION_COMMAND: &'static str = "sectionaward";
}
