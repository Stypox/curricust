use std::io::Write;
use std::path::Path;

use crate::attr::context::Context;
use crate::attr::parse::try_parse_group;
use crate::attr::text_with_attributes::TextWithAttributes;
use crate::item::talk_item::TalkItem;
use crate::writer::europass_xml_writer::{ATTACHMENT_XML_FILENAME, MyXmlUtils};
use crate::writer::latex_writer::LatexWriter;
use crate::writer::{AllWriters, write::MyWrite};
use crate::util::file::{include_file, include_file_with_context};
use crate::util::yaml::YamlConversions;
use multimap::MultiMap;
use xml_builder::{XML, XMLBuilder, XMLElement, XMLError, XMLVersion};
use yaml_rust::Yaml;

use crate::header::HeaderElement;
use crate::element::section::SectionElement;
use crate::item::award_item::AwardItem;
use crate::item::education_item::EducationItem;
use crate::item::job_item::JobItem;
use crate::item::project_item::ProjectItem;
use crate::item::SectionItem;

#[derive(Debug)]
pub struct BaseElement {
    header: HeaderElement,
    sections: Vec<Box<dyn AllWriters>>,
    language: Option<String>,
}

impl BaseElement {
    fn parse_dictionary(
        dictionary: &mut MultiMap<String, TextWithAttributes>,
        hash: Yaml,
    ) -> Result<(), String> {
        let hash = hash.einto_hash()?;
        for (key, value) in hash.into_iter() {
            let (key, value) = TextWithAttributes::new_yaml(key, value)?;
            dictionary.insert(key, value);
        }
        Ok(())
    }

    fn parse_section<T>(
        sections: &mut Vec<Box<dyn AllWriters>>,
        ctx: &Context,
        value: Yaml,
    ) -> Result<(), String>
    where
        T: SectionItem + 'static,
        SectionElement<T>: AllWriters,
    {
        sections.push(Box::new(SectionElement::<T>::parse(ctx, false, value)?));
        Ok(())
    }

    fn parse_include_section<T>(
        sections: &mut Vec<Box<dyn AllWriters>>,
        ctx: &Context,
        root: &Path,
        value: Yaml,
    ) -> Result<(), String>
    where
        T: SectionItem + 'static,
        SectionElement<T>: AllWriters,
    {
        let (override_ctx, value) = include_file_with_context(root, ctx.clone(), value)?;
        sections.push(Box::new(SectionElement::<T>::parse(&override_ctx, false, value)?));
        Ok(())
    }

    pub fn new(root: &Path, array: Yaml) -> Result<BaseElement, String> {
        let array = array.einto_vec()?;
        let mut ctx = Context::default();
        let mut header = HeaderElement::builder();
        let mut sections: Vec<Box<dyn AllWriters>> = vec![];

        for yaml in array {
            let (key, value) = yaml.einto_single_element_hash()?;

            let Some(value) = try_parse_group(&mut ctx, &key, value)? else {
                continue;
            };

            match key.as_str() {
                "dictionary" => Self::parse_dictionary(&mut ctx.dictionary, value)?,
                "include-dictionary" => {
                    Self::parse_dictionary(&mut ctx.dictionary, include_file(root, value)?)?
                }
                "header" => HeaderElement::parse(&mut header, &ctx, root, value)?,
                "include-header" => HeaderElement::parse(&mut header, &ctx, root, include_file(root, value)?)?,
                "section-education" => {
                    Self::parse_section::<EducationItem>(&mut sections, &ctx, value)?
                }
                "include-section-education" => {
                    Self::parse_include_section::<EducationItem>(&mut sections, &ctx, root, value)?
                }
                "section-award" => Self::parse_section::<AwardItem>(&mut sections, &ctx, value)?,
                "include-section-award" => {
                    Self::parse_include_section::<AwardItem>(&mut sections, &ctx, root, value)?
                }
                "section-job" => Self::parse_section::<JobItem>(&mut sections, &ctx, value)?,
                "include-section-job" => {
                    Self::parse_include_section::<JobItem>(&mut sections, &ctx, root, value)?
                }
                "section-project" => {
                    Self::parse_section::<ProjectItem>(&mut sections, &ctx, value)?
                }
                "include-section-project" => {
                    Self::parse_include_section::<ProjectItem>(&mut sections, &ctx, root, value)?
                }
                "section-talk" => {
                    Self::parse_section::<TalkItem>(&mut sections, &ctx, value)?
                }
                "include-section-talk" => {
                    Self::parse_include_section::<TalkItem>(&mut sections, &ctx, root, value)?
                }
                _ => return Err(format!("Base element can't have children of type {key}")),
            }
        }

        let header = header.build(&ctx)?;
        Ok(BaseElement { header, sections, language: ctx.get_base_attr() })
    }

    pub fn latex_write(&self, f: &mut MyWrite, attach_europass_xml: bool) -> std::io::Result<()> {
        writeln!(f, r"\documentclass[11pt]{{resumecvrusttemplate}}")?;
        writeln!(f, r"\usepackage{{multicol}}")?;
        if attach_europass_xml {
            writeln!(f, "\\usepackage{{embedfile}}")?;
        }
        writeln!(f, r"\setlength{{\columnsep}}{{0mm}}")?;
        writeln!(f, "\\begin{{document}}\n")?;
        if attach_europass_xml {
            writeln!(f, "\\embedfile{{{ATTACHMENT_XML_FILENAME}}}")?;
        }
        self.header.latex_write(f)?;
        writeln!(f, "{{}}{{}}{{")?;
        for section in &self.sections {
            section.latex_write(f)?;
            writeln!(f)?;
        }
        writeln!(f, "}}")?;
        writeln!(f, r"\end{{document}}")?;
        Ok(())
    }

    pub fn to_europass_xml(&self) -> Result<XML, XMLError> {
        let candidate = XMLElement::new("Candidate")
            .my_add_attribute("xsi:schemaLocation", "http://www.europass.eu/1.0 Candidate.xsd")
            .my_add_attribute("xmlns", "http://www.europass.eu/1.0")
            .my_add_attribute("xmlns:oa", "http://www.openapplications.org/oagis/9")
            .my_add_attribute("xmlns:eures", "http://www.europass_eures.eu/1.0")
            .my_add_attribute("xmlns:hr", "http://www.hr-xml.org/3")
            .my_add_attribute("xmlns:xsi", "http://www.w3.org/2001/XMLSchema-instance")
            .my_add_child(
                XMLElement::new("hr:DocumentID")
                    .my_add_attribute("schemeID", "Test-0001")
                    .my_add_attribute("schemeName", "DocumentIdentifier")
                    .my_add_attribute("schemeAgencyName", "EUROPASS")
                    .my_add_attribute("schemeVersionID", "4.0")
            )?
            .my_add_child(self.header.to_candidate_supplier_xml()?)?
            .my_add_child(self.header.to_candidate_person_xml()?)?
            .my_add_child({
                let mut candidate_profile = XMLElement::new("CandidateProfile")
                    .my_add_attribute("languageCode", self.language.as_deref().unwrap_or("en")) // TODO
                    .my_add_child(
                        XMLElement::new("hr:ID")
                            .my_add_attribute("schemeID", "Test-0001")
                            .my_add_attribute("schemeName", "CandidateProfileID")
                            .my_add_attribute("schemeAgencyName", "EUROPASS")
                            .my_add_attribute("schemeVersionID", "1.0")
                    )?;

                candidate_profile = self.header.extend_candidate_profile_xml(candidate_profile)?;

                let mut elements = Vec::<(&'static str, XMLElement)>::new();
                for section in &self.sections {
                    elements.extend(section.to_europass_xml()?);
                }

                while let Some((key, _)) = elements.first().cloned() {
                    let (elements_with_key, remaining) = elements.into_iter().partition(|e| e.0 == key);
                    elements = remaining;

                    let mut parent = XMLElement::new(key);
                    for (_, element) in elements_with_key {
                        parent = parent.my_add_child(element)?;
                    }
                    candidate_profile = candidate_profile.my_add_child(parent)?;
                }

                candidate_profile
            })?;

        let mut xml = XMLBuilder::new()
            .version(XMLVersion::XML1_0)
            .encoding("UTF-8".into())
            .build();
        xml.set_root_element(candidate);
        Ok(xml)
    }
}
