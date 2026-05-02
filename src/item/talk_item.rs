use chrono::NaiveDate;
use curricust_proc_macro::{CvElementBuilder, CvSectionItem};
use xml_builder::{XMLElement, XMLError};


use crate::writer::{
    europass_xml_writer::{EuropassXmlWriter, MyXmlUtils}, latex_writer::{LatexWriter, SectionItemLatexWriter, write_latex_command_call}, markdown_utils::{extract_markdown_uri, markdown_to_html, markdown_to_plaintext}, write::MyWrite
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

impl EuropassXmlWriter for TalkItem {
    fn to_europass_xml(&self) -> Result<Vec<(&'static str, XMLElement)>, XMLError> {
        // attempt to parse the date, just in case
        let date = if let Ok(date) = NaiveDate::parse_from_str(&self.when, "%d/%m/%Y") {
            Some(date.format("%Y-%m-%d").to_string())
        } else {
            None
        };

        // note: the more fitting category would be SpeakingHistory but Europass uses ConferencesAndSeminars...
        Ok(vec![("ConferencesAndSeminars", XMLElement::new("ConferenceAndSeminar")
            .my_add_text_child("Title", &markdown_to_plaintext(&self.name))?
            .my_add_child(
                XMLElement::new("Date")
                    .my_add_child_if_nonempty(&date, |start|
                        XMLElement::new("StartDate")
                            .my_add_text_child("hr:FormattedDateTime", start)
                    )?
                    .my_add_child_if_nonempty(&date, |end|
                        XMLElement::new("EndDate")
                            .my_add_text_child("hr:FormattedDateTime", end)
                    )?
                    .my_add_text_child("Ongoing", "false")?
                    .my_add_text_child_if_nonempty("Description", &self.when)?
            )?
            .my_add_text_child("Location", &markdown_to_html(&self.event))?
            .my_add_text_child_if_nonempty("Link", extract_markdown_uri(&self.name).as_deref().unwrap_or(""))?)])
    }
}
