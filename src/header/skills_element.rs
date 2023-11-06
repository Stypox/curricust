use resume_cv_proc_macro::CvElementBuilder;
use yaml_rust::Yaml;

use crate::{writer::latex_writer::{LatexWriter, write_latex_command_call}, attr::{context::Context, text_with_attributes::TextWithAttributes}, util::yaml::YamlConversions};

#[derive(Debug, CvElementBuilder)]
pub struct SkillsElement {
    #[cv_element_builder(text_with_attributes)]
    pub title: String,
    #[cv_element_builder(text_with_attributes)]
    pub skills: String,
}

impl SkillsElement {
    pub fn parse(ctx: &Context, hash: Yaml) -> Result<Self, String> {
        let hash = hash.einto_hash()?;
        let mut builder = SkillsElement::builder();

        for (key, value) in hash {
            let (key, value) = TextWithAttributes::new_yaml(key, value)?;
            match key.as_str() {
                "title" => builder.add_title(value),
                "skills" => builder.add_skills(value),
                _ => return Err(format!("Unknown key in header: {key}")),
            };
        }
        builder.build(ctx)
    }
}

impl LatexWriter for SkillsElement {
    fn latex_write(&self, f: &mut crate::writer::MyWrite) -> std::io::Result<()> {
        write_latex_command_call(f, "sectionskills", &[&self.title, &self.skills])
    }
}
