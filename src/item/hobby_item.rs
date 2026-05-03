use curricust_proc_macro::{CvElementBuilder, CvSectionItem};
use xml_builder::{XMLElement, XMLError};

use crate::writer::{
    europass_xml_writer::{EuropassXmlWriter, MyXmlUtils}, latex_writer::{LatexWriter, SectionItemLatexWriter, write_latex_command_call}, markdown_utils::{extract_markdown_uris, markdown_to_html, markdown_to_plaintext}, write::MyWrite
};

#[derive(Debug, CvElementBuilder, CvSectionItem)]
pub struct HobbyItem {
    #[cv_element_builder(text_with_attributes)]
    pub title: String,
    #[cv_element_builder(text_with_attributes)]
    pub institution: Option<String>,
    #[cv_element_builder(text_with_attributes)]
    pub when: Option<String>,

    // only used in XML
    pub start: Option<String>,
    pub end: Option<String>,

    #[cv_element_builder(text_with_attributes)]
    pub details: Option<String>,
}

impl LatexWriter for HobbyItem {
    fn latex_write(&self, f: &mut MyWrite) -> std::io::Result<()> {
        write_latex_command_call(
            f,
            "itemhobby",
            &[
                &self.title,
                self.institution.as_deref().unwrap_or(""),
                self.when.as_deref().unwrap_or(""),
                self.details.as_deref().unwrap_or(""),
            ],
        )
    }
}

impl SectionItemLatexWriter for HobbyItem {
    const SECTION_COMMAND: &'static str = "sectioneducation";
}

impl EuropassXmlWriter for HobbyItem {
    fn to_europass_xml(&self) -> Result<Vec<(&'static str, XMLElement)>, XMLError> {
        let (title, links) = if let Some(institution) = &self.institution {
            (&format!("{} @ {}", self.title, markdown_to_plaintext(institution)), extract_markdown_uris(institution))
        } else {
            (&self.title, vec![])
        };

        let mut hobby_or_interest = XMLElement::new("HobbyOrInterest")
            .my_add_text_child("Title", title)?
            .my_add_text_child_if_nonempty("Description", &markdown_to_html(self.details.as_deref().unwrap_or("")))?
            .my_add_child(
                XMLElement::new("Date")
                    .my_add_child_if_nonempty(&self.start, |start|
                        XMLElement::new("StartDate")
                            .my_add_text_child("hr:FormattedDateTime", start)
                    )?
                    .my_add_child_if_nonempty(&self.end, |end|
                        XMLElement::new("EndDate")
                            .my_add_text_child("hr:FormattedDateTime", end)
                    )?
                    .my_add_text_child("Ongoing", if self.start.is_some() && self.end.is_none() { "true" } else { "false" })?
                    .my_add_text_child_if_nonempty("Description", self.when.as_deref().unwrap_or(""))?
            )?;


        for link in links {
            hobby_or_interest = hobby_or_interest.my_add_text_child("Link", &link)?;
        }

        Ok(vec![("HobbiesAndInterests", hobby_or_interest)])
    }
}
