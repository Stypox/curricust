pub mod summary_element;
pub mod skills_element;

use std::{io::Write, path::Path};

use resume_cv_proc_macro::CvElementBuilder;


use yaml_rust::Yaml;

use crate::{
    attr::{text_with_attributes::TextWithAttributes, context::Context},
    writer::{latex_writer::{LatexWriter, write_latex_command_call}, write::MyWrite},
    util::{yaml::YamlConversions, file::include_file},
};

use self::{summary_element::SummaryElement, skills_element::SkillsElement};

#[derive(Debug, CvElementBuilder)]
pub struct HeaderElement {
    #[cv_element_builder(text_with_attributes)]
    name: String,
    #[cv_element_builder(text_with_attributes)]
    career: Option<String>,

    #[cv_element_builder(text_with_attributes)]
    email: Option<String>,
    #[cv_element_builder(text_with_attributes)]
    phone: Option<String>,
    #[cv_element_builder(text_with_attributes)]
    location: Option<String>,

    #[cv_element_builder(text_with_attributes)]
    website: Option<String>,
    #[cv_element_builder(text_with_attributes)]
    github: Option<String>,
    #[cv_element_builder(text_with_attributes)]
    linkedin: Option<String>,

    summary: Option<SummaryElement>,
    skills: Option<SkillsElement>,
}

impl HeaderElement {
    fn try_parse_elements(header: &mut HeaderElementBuilder,
        ctx: &Context,
        root: &Path, key: &str, value: Yaml) -> Result<Option<Yaml>, String> {
        match key {
            "summary" => header.summary(SummaryElement::parse(ctx, value)?),
            "include-summary" => header.summary(SummaryElement::parse(ctx, include_file(root, value)?)?),
            "skills" => header.skills(SkillsElement::parse(ctx, value)?),
            "include-skills" => header.skills(SkillsElement::parse(ctx, include_file(root, value)?)?),
            _ => return Ok(Some(value)),
        };
        Ok(None)
    }

    pub fn parse(header: &mut HeaderElementBuilder,
        ctx: &Context,
        root: &Path, hash: Yaml) -> Result<(), String> {
        let hash = hash.einto_hash()?;
        for (key, value) in hash {
            let key = key.einto_string()?;
            let Some(value) = Self::try_parse_elements(header, ctx, root, &key, value)? else {
                continue;
            };

            let (key, value) = TextWithAttributes::new_string(key, value)?;
            match key.as_str() {
                "name" => header.add_name(value),
                "career" => header.add_career(value),
                "email" => header.add_email(value),
                "phone" => header.add_phone(value),
                "location" => header.add_location(value),
                "website" => header.add_website(value),
                "github" => header.add_github(value),
                "linkedin" => header.add_linkedin(value),
                _ => return Err(format!("Unknown key in header: {key}")),
            };
        }
        Ok(())
    }
}

impl LatexWriter for HeaderElement {
    fn latex_write(&self, f: &mut MyWrite) -> std::io::Result<()> {
        write_latex_command_call(
            f,
            "cv",
            &[
                &self.name,
                self.career.as_deref().unwrap_or(""),
                self.email.as_deref().unwrap_or(""),
                self.phone.as_deref().unwrap_or(""),
                self.location.as_deref().unwrap_or(""),
                self.website.as_deref().unwrap_or(""),
                self.github.as_deref().unwrap_or(""),
                self.linkedin.as_deref().unwrap_or(""),
            ],
        )?;

        write!(f, "{{")?;
        if let Some(summary) = &self.summary {
            summary.latex_write(f)?;
        }
        write!(f, "}}{{")?;
        if let Some(skills) = &self.skills {
            skills.latex_write(f)?;
        }
        write!(f, "}}")?;

        Ok(())
    }
}
