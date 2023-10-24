use resume_cv_proc_macro::{CvElementBuilder, CvRMarkdownItem, CvSectionItem};

use crate::printers::{cv_developer_latex_printer::CvDeveloperLatexSectionItem, Writer, markdown_to_latex::write_markdown};

#[derive(Debug, CvElementBuilder, CvRMarkdownItem, CvSectionItem)]
pub struct JobItem {
    #[cv_element_builder(text_with_attributes)]
    pub dates: String,
    #[cv_element_builder(text_with_attributes)]
    pub institution: String,
}

impl CvDeveloperLatexSectionItem for JobItem {
    fn cvdl_print_left(&self, f: &mut Writer) -> std::io::Result<()> {
        write_markdown(f, &self.dates)
    }

    fn cvdl_print_heading(&self, _f: &mut Writer) -> std::io::Result<()> {
        Ok(())
    }

    fn cvdl_print_qualifier(&self, f: &mut Writer) -> std::io::Result<()> {
        write_markdown(f, &self.institution)
    }

    fn cvdl_print_description(&self, _f: &mut Writer) -> std::io::Result<()> {
        Ok(())
    }
}
