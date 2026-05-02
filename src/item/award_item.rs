use chrono::NaiveDate;
use curricust_proc_macro::{CvElementBuilder, CvSectionItem};
use xml_builder::{XMLElement, XMLError};


use crate::writer::{
    europass_xml_writer::{EuropassXmlWriter, MyXmlUtils}, latex_writer::{LatexWriter, SectionItemLatexWriter, write_latex_command_call}, markdown_utils::{extract_markdown_uri, markdown_to_plaintext}, write::MyWrite
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

impl LatexWriter for AwardItem {
    fn latex_write(&self, f: &mut MyWrite) -> std::io::Result<()> {
        write_latex_command_call(
            f,
            "itemaward",
            &[&self.name, &self.when, self.grade.as_deref().unwrap_or("")],
        )
    }
}

impl SectionItemLatexWriter for AwardItem {
    const SECTION_COMMAND: &'static str = "sectionaward";
}

impl EuropassXmlWriter for AwardItem {
    fn to_europass_xml(&self) -> Result<Vec<(&'static str, XMLElement)>, XMLError> {
        // attempt to parse the date, just in case
        let date = if let Ok(date) = NaiveDate::parse_from_str(&self.when, "%d/%m/%Y") {
            &date.format("%Y-%m-%d").to_string()
        } else {
            &self.when
        };

        Ok(vec![("Certifications", XMLElement::new("Certification")
            .my_add_text_child("hr:CertificationName", &markdown_to_plaintext(&self.name))?
            .my_add_child(
                XMLElement::new("eures:FirstIssuedDate")
                    .my_add_text_child("hr:FormattedDateTime", date)?
            )?
            // no better known place for the grade
            .my_add_text_child_if_nonempty("oa:Description", &self.grade.as_deref().map(markdown_to_plaintext).unwrap_or("".to_string()))?
            .my_add_text_child_if_nonempty("Link", &self.grade.as_deref().and_then(extract_markdown_uri).unwrap_or("".to_string()))?)])
    }
}

