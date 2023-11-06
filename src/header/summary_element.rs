use resume_cv_proc_macro::CvElementBuilder;
use yaml_rust::Yaml;

use crate::{writer::latex_writer::{LatexWriter, write_latex_command_call}, attr::{text_with_attributes::TextWithAttributes, context::Context}, util::yaml::YamlConversions};

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
}

impl LatexWriter for SummaryElement {
    fn latex_write(&self, f: &mut crate::writer::Writer) -> std::io::Result<()> {
        write_latex_command_call(f, "sectionsummary", &[&self.title, &self.summary])
    }
}
