use curricust_proc_macro::CvElementBuilder;
use xml_builder::{XMLElement, XMLError};
use yaml_rust::Yaml;

use crate::{attr::{context::Context, text_with_attributes::TextWithAttributes}, util::yaml::YamlConversions, writer::{europass_xml_writer::build_xml_text_element, latex_writer::{LatexWriter, write_latex_command_call}, markdown_utils::markdown_to_html, write::MyWrite}};

#[derive(Debug, CvElementBuilder)]
pub struct SummaryElement {
    #[cv_element_builder(text_with_attributes)]
    pub title: String,
    #[cv_element_builder(text_with_attributes)]
    pub summary: String,
}

impl SummaryElement {
    pub fn parse(ctx: &Context, hash: Yaml) -> Result<Self, String> {
        let hash = hash.einto_hash()?;
        let mut builder = SummaryElement::builder();

        for (key, value) in hash {
            let (key, value) = TextWithAttributes::new_yaml(key, value)?;
            match key.as_str() {
                "title" => builder.add_title(value),
                "summary" => builder.add_summary(value),
                _ => return Err(format!("Unknown key in header: {key}")),
            };
        }
        builder.build(ctx)
    }

    pub fn to_experience_summary_xml(&self) -> Result<XMLElement, XMLError> {
        build_xml_text_element("ExperienceSummary", &markdown_to_html(&self.summary))
    }
}

impl LatexWriter for SummaryElement {
    fn latex_write(&self, f: &mut MyWrite) -> std::io::Result<()> {
        write_latex_command_call(f, "sectionsummary", &[&self.title, &self.summary])
    }
}
