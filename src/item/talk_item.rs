use resume_cv_proc_macro::{CvElementBuilder, CvSectionItem};


use crate::printers::{
    latex_printer::{write_latex_command_call, LatexPrinter, SectionItemLatexPrinter},
    Writer,
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

impl LatexPrinter for TalkItem {
    fn latex_print(&self, f: &mut Writer) -> std::io::Result<()> {
        write_latex_command_call(
            f,
            "itemtalk",
            &[&self.name, &self.event, &self.when],
        )
    }
}

impl SectionItemLatexPrinter for TalkItem {
    const SECTION_COMMAND: &'static str = "sectiontalk";
}
