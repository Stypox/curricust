use curricust_proc_macro::{CvElementBuilder, CvSectionItem};
use xml_builder::{XMLElement, XMLError};

use crate::writer::{
    europass_xml_writer::{EuropassXmlWriter, MyXmlUtils, build_xml_text_element}, latex_writer::{LatexWriter, SectionItemLatexWriter, write_latex_command_call}, markdown_utils::{extract_markdown_uri, markdown_to_html, markdown_to_plaintext}, write::MyWrite
};

#[derive(Debug, CvElementBuilder, CvSectionItem)]
pub struct JobItem {
    #[cv_element_builder(text_with_attributes)]
    pub role: String,
    #[cv_element_builder(text_with_attributes)]
    pub company: String,
    #[cv_element_builder(text_with_attributes)]
    pub where_: Option<String>,
    #[cv_element_builder(text_with_attributes)]
    pub when: String,

    // only used in XML
    pub start: String,
    pub end: Option<String>,

    #[cv_element_builder(text_with_attributes)]
    pub topics: Option<String>,
    #[cv_element_builder(text_with_attributes)]
    pub details: Option<String>,
}

impl LatexWriter for JobItem {
    fn latex_write(&self, f: &mut MyWrite) -> std::io::Result<()> {
        write_latex_command_call(
            f,
            "itemjob",
            &[
                &self.role,
                &self.company,
                self.where_.as_deref().unwrap_or(""),
                &self.when,
                self.topics.as_deref().unwrap_or(""),
                self.details.as_deref().unwrap_or(""),
            ],
        )
    }
}

impl SectionItemLatexWriter for JobItem {
    const SECTION_COMMAND: &'static str = "sectionjob";
}

impl EuropassXmlWriter for JobItem {
    fn to_europass_xml(&self) -> Result<Vec<(&'static str, XMLElement)>, XMLError> {
        // unfortunately the topics need to go together with the description
        let description = if let Some(topics) = &self.topics {
            if let Some(details) = &self.details {
                &Some(format!("{}\n<br>{}", details, topics))
            } else {
                &self.topics
            }
        } else {
            &self.details
        };

        Ok(vec![("EmploymentHistory", XMLElement::new("EmployerHistory")
            .my_add_text_child_if_nonempty("hr:OrganizationName", &markdown_to_plaintext(&self.company))?
            .my_add_child(
                XMLElement::new("OrganizationContact")
                    .my_add_child(
                        XMLElement::new("Communication")
                            .my_add_child(
                                // include the whole tree even if where_ is empty, that's what Europass does
                                XMLElement::new("Address")
                                    .my_add_text_child_if_nonempty("oa:AddressLine", self.where_.as_deref().unwrap_or(""))?
                            )?
                    )?
                    .my_add_child_if_nonempty(&extract_markdown_uri(&self.company), |uri|
                        XMLElement::new("Communication")
                            .my_add_text_child("ChannelCode", "Web")?
                            .my_add_text_child("oa:URI", uri)
                    )?
            )?
            .my_add_child(
                XMLElement::new("PositionHistory")
                    .my_add_child(
                        build_xml_text_element("PositionTitle", &self.role)?
                            .my_add_attribute("typeCode", "FREETEXT")
                    )?
                    .my_add_child(
                        XMLElement::new("eures:EmploymentPeriod")
                            .my_add_child(
                                XMLElement::new("eures:StartDate")
                                    .my_add_text_child("hr:FormattedDateTime", &self.start)?
                            )?
                            .my_add_child_if_nonempty(&self.end, |end|
                                XMLElement::new("eures:EndDate")
                                    .my_add_text_child("hr:FormattedDateTime", end)
                            )?
                            .my_add_text_child("hr:CurrentIndicator", if self.end.is_some() { "false" } else { "true" })?
                    )?
                    .my_add_child_if_nonempty(description, |description|
                        build_xml_text_element("oa:Description", &markdown_to_html(description))
                    )?
                    .my_add_text_child_if_nonempty("City", self.where_.as_deref().unwrap_or(""))?
                    .my_add_text_child_if_nonempty("Country", self.where_.as_deref().unwrap_or(""))?
            )?)])
    }
}
