use std::sync::LazyLock;

use curricust_proc_macro::{CvElementBuilder, CvSectionItem};
use regex::Regex;
use xml_builder::{XMLElement, XMLError};


use crate::writer::{
    europass_xml_writer::{EuropassXmlWriter, MyXmlUtils, build_xml_text_element}, latex_writer::{LatexWriter, SectionItemLatexWriter, write_latex_command_call}, markdown_utils::{markdown_to_html, markdown_to_plaintext}, write::MyWrite
};

#[derive(Debug, CvElementBuilder, CvSectionItem)]
pub struct EducationItem {
    #[cv_element_builder(text_with_attributes)]
    pub degree: String,
    #[cv_element_builder(text_with_attributes)]
    pub institution: String,
    #[cv_element_builder(text_with_attributes)]
    pub when: String,

    // only used in XML
    pub start: String,
    pub end: Option<String>,

    #[cv_element_builder(text_with_attributes)]
    pub grade: Option<String>,
    #[cv_element_builder(text_with_attributes)]
    pub details: Option<String>,
}

impl LatexWriter for EducationItem {
    fn latex_write(&self, f: &mut MyWrite) -> std::io::Result<()> {
        write_latex_command_call(
            f,
            "itemeducation",
            &[
                &self.degree,
                &self.institution,
                &self.when,
                self.grade.as_deref().unwrap_or(""),
                self.details.as_deref().unwrap_or(""),
            ],
        )
    }
}

impl SectionItemLatexWriter for EducationItem {
    const SECTION_COMMAND: &'static str = "sectioneducation";
}

static COLON_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(":[ \\n]+").unwrap());

impl EuropassXmlWriter for EducationItem {
    fn to_europass_xml(&self) -> Result<Vec<(&'static str, XMLElement)>, XMLError> {
        let thesis = if let Some(thesis) = &self.details {
            let thesis = markdown_to_plaintext(thesis);
            let thesis = thesis.split("\n\n").collect::<Vec<_>>();
            // cannot crash, split returns >=1 items; the last one is languages
            let thesis = thesis.last().unwrap();
            let thesis = COLON_REGEX.splitn(thesis, 2).map(|e| e.to_string()).collect::<Vec<_>>();
            if thesis.first().is_some_and(|t| t.len() < 10) {
                thesis.get(1).cloned()
            } else {
                None // too long word for "thesis", probably not thesis
            }
        } else {
            None
        };

        Ok(vec![("EducationHistory", XMLElement::new("EducationOrganizationAttendance")
            .my_add_text_child_if_nonempty("hr:OrganizationName", &self.institution)?
            .my_add_child(
                XMLElement::new("EducationDegree")
                    .my_add_text_child_if_nonempty("hr:DegreeName", &self.degree)?
                    .my_add_child_if_nonempty(&self.grade, |grade|
                        XMLElement::new("FinalGrade")
                            .my_add_text_child("hr:ScoreText", grade)
                    )?
                    .my_add_text_child_if_nonempty("Thesis", thesis.as_deref().unwrap_or(""))?
                    .my_add_child_if_nonempty(&self.details, |details|
                        build_xml_text_element("OccupationalSkillsCovered", &markdown_to_html(details))
                    )?

            )?
            .my_add_child(
                XMLElement::new("AttendancePeriod")
                    .my_add_child_if_nonempty(&Some(self.start.clone()), |start|
                        XMLElement::new("StartDate")
                            .my_add_text_child("hr:FormattedDateTime", start)
                    )?
                    .my_add_child_if_nonempty(&self.end, |end|
                        XMLElement::new("EndDate")
                            .my_add_text_child("hr:FormattedDateTime", end)
                    )?
                    .my_add_text_child("Ongoing", if self.end.is_some() { "false" } else { "true" })?
                    .my_add_text_child_if_nonempty("Description", &self.when)?
            )?)])
    }
}
