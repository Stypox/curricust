use std::sync::LazyLock;

use curricust_proc_macro::CvElementBuilder;
use regex::Regex;
use xml_builder::{XMLElement, XMLError};
use yaml_rust::Yaml;

use crate::{attr::{context::Context, text_with_attributes::TextWithAttributes}, util::yaml::YamlConversions, writer::{europass_xml_writer::{MyXmlUtils, build_xml_text_element}, latex_writer::{LatexWriter, write_latex_command_call}, markdown_utils::markdown_to_plaintext, write::MyWrite}};

#[derive(Debug, CvElementBuilder)]
pub struct SkillsElement {
    #[cv_element_builder(text_with_attributes)]
    pub title: String,
    #[cv_element_builder(text_with_attributes)]
    pub skills: String,
}

static COLON_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(":[ \\n]+").unwrap());
static COMMA_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(",[ \\n]+").unwrap());

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

    // appears in newer Europass PDFs
    pub fn to_skills_xml(&self) -> Result<XMLElement, XMLError> {
        let sections = markdown_to_plaintext(&self.skills);
        let sections = sections.split("\n\n").collect::<Vec<_>>();
        // cannot crash, split returns >=1 items; the last one is languages
        let sections = &sections[..sections.len()-1];

        let mut skills = XMLElement::new("Skills");

        for section in sections {
            let v = COLON_REGEX.splitn(section, 2).collect::<Vec<_>>();
            let (title, list) = if let (Some(title), Some(list)) = (v.first(), v.get(1)) {
                (*title, *list)
            } else {
                ("", *section)
            };

            let mut group = XMLElement::new("SkillsGroup")
                .my_add_text_child_if_nonempty("Title", title)?;

            for item in COMMA_REGEX.split(list) {
                group = group.my_add_child_if_nonempty(&Some(item.to_string()), |item|
                    XMLElement::new("PersonCompetency")
                        .my_add_text_child("hr:TaxonomyID", "Digital_Skill")?
                        .my_add_text_child("hr:CompetencyName", item)
                )?;
            }

            skills = skills.my_add_child(group)?
        }

        Ok(skills)
    }

    // appears in older Europass PDFs
    pub fn to_digital_skills_xml(&self) -> Result<XMLElement, XMLError> {
        let sections = markdown_to_plaintext(&self.skills);
        let sections = sections.split("\n\n").collect::<Vec<_>>();
        // cannot crash, split returns >=1 items; the last one is languages
        let sections = &sections[..sections.len()-1];

        let mut digital_skills = XMLElement::new("DigitalSkills");

        for section in sections {
            let v = COLON_REGEX.splitn(section, 2).collect::<Vec<_>>();
            let (title, list) = if let (Some(title), Some(list)) = (v.first(), v.get(1)) {
                (*title, *list)
            } else {
                ("", *section)
            };

            let mut group = XMLElement::new("DigitalSkillsGroup")
                .my_add_text_child_if_nonempty("Title", title)?;

            for item in COMMA_REGEX.split(list) {
                group = group.my_add_text_child_if_nonempty("DigitalSkill", item)?;
            }

            digital_skills = digital_skills.my_add_child(group)?
        }

        Ok(digital_skills)
    }

    pub fn to_personal_qualifications_xml(&self) -> Result<XMLElement, XMLError> {
        let sections = markdown_to_plaintext(&self.skills);
        // cannot crash, split returns >=1 items; the last one is languages
        let section = sections.split("\n\n").last().unwrap();
        let languages = COMMA_REGEX.split(section);

        let mut personal_qualifications = XMLElement::new("PersonQualifications");

        for language in languages {
            let v = COLON_REGEX.splitn(language, 2).collect::<Vec<_>>();
            let (lang, score) = if let (Some(lang), Some(score)) = (v.first(), v.get(1)) {
                (*lang, *score)
            } else {
                (language, "")
            };

            let mut person_competency = XMLElement::new("PersonCompetency")
                .my_add_child(
                    build_xml_text_element("CompetencyID", lang)?
                        .my_add_attribute("schemeName", "FREE_TEXT")
                )?
                .my_add_text_child("hr:TaxonomyID", "language")?;

            if !score.is_empty() {
                for code in [
                    "CEF-Understanding-Listening",
                    "CEF-Understanding-Reading",
                    "CEF-Speaking-Interaction",
                    "CEF-Speaking-Production",
                    "CEF-Writing-Production"
                ] {
                    person_competency = person_competency.my_add_child(
                        XMLElement::new("eures:CompetencyDimension")
                            .my_add_text_child("hr:CompetencyDimensionTypeCode", code)?
                            .my_add_child(
                                XMLElement::new("eures:Score")
                                    .my_add_text_child("hr:ScoreText", score)?
                            )?
                    )?;
                }
            }

            personal_qualifications = personal_qualifications.my_add_child(person_competency)?;
        }

        Ok(personal_qualifications)
    }
}

impl LatexWriter for SkillsElement {
    fn latex_write(&self, f: &mut MyWrite) -> std::io::Result<()> {
        write_latex_command_call(f, "sectionskills", &[&self.title, &self.skills])
    }
}
