use curricust_proc_macro::{CvElementBuilder, CvSectionItem};
use xml_builder::{XMLElement, XMLError};


use crate::writer::{
    europass_xml_writer::{EuropassXmlWriter, MyXmlUtils}, latex_writer::{LatexWriter, SectionItemLatexWriter, write_latex_command_call}, markdown_utils::{extract_markdown_uris, markdown_to_html}, write::MyWrite
};

#[derive(Debug, CvElementBuilder, CvSectionItem)]
pub struct ProjectItem {
    #[cv_element_builder(text_with_attributes)]
    pub name: String,
    #[cv_element_builder(text_with_attributes)]
    pub technologies: String,
    #[cv_element_builder(text_with_attributes)]
    pub links: Option<String>,
    #[cv_element_builder(text_with_attributes)]
    pub when: Option<String>,
    #[cv_element_builder(text_with_attributes)]
    pub details: Option<String>,
}

impl LatexWriter for ProjectItem {
    fn latex_write(&self, f: &mut MyWrite) -> std::io::Result<()> {
        write_latex_command_call(
            f,
            "itemproject",
            &[
                &self.name,
                &self.technologies,
                self.links.as_deref().unwrap_or(""),
                self.when.as_deref().unwrap_or(""),
                self.details.as_deref().unwrap_or(""),
            ],
        )
    }
}

impl SectionItemLatexWriter for ProjectItem {
    const SECTION_COMMAND: &'static str = "sectionproject";
}

impl EuropassXmlWriter for ProjectItem {
    fn to_europass_xml(&self) -> Result<Vec<(&'static str, XMLElement)>, XMLError> {
        let description = [&self.details.as_ref(), &Some(&self.technologies), &self.links.as_ref()]
            .iter()
            .filter_map(|f| f.cloned())
            .filter(|f| !f.is_empty())
            .reduce(|a, b| format!("{a}\n<br>{b}"));

        let mut project = XMLElement::new("Project")
            .my_add_text_child("Title", &markdown_to_html(&self.name))?
            .my_add_text_child_if_nonempty("Description", &description.as_deref().map(markdown_to_html).unwrap_or("".to_string()))?
            .my_add_child_if_nonempty(&self.when, |when|
                XMLElement::new("Date")
                    .my_add_text_child("Ongoing", "false")?
                    .my_add_text_child_if_nonempty("Description", when)
            )?;

        if let Some(links) = &self.links {
            for link in extract_markdown_uris(links) {
                project = project.my_add_text_child("Link", &link)?;
            }
        }

        Ok(vec![("Projects", project)])
    }
}
