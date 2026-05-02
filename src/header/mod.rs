pub mod summary_element;
pub mod skills_element;

use std::{fs, io::Write, path::{Path, PathBuf}};

use base64::{Engine, prelude::BASE64_STANDARD};
use curricust_proc_macro::CvElementBuilder;


use phonelib::PhoneNumber;
use xml_builder::{XMLElement, XMLError};
use yaml_rust::Yaml;

use crate::{
    attr::{context::Context, text_with_attributes::TextWithAttributes}, util::{file::include_file, yaml::YamlConversions}, writer::{europass_xml_writer::{MyXmlUtils, build_xml_text_element}, latex_writer::{LatexWriter, write_latex_command_call}, markdown_utils::{extract_markdown_uri_or_original, markdown_to_plaintext}, write::MyWrite}
};

use self::{summary_element::SummaryElement, skills_element::SkillsElement};

#[derive(Debug, CvElementBuilder)]
pub struct HeaderElement {
    #[cv_element_builder(text_with_attributes)]
    name: String,
    #[cv_element_builder(text_with_attributes)]
    surname: String,
    #[cv_element_builder(text_with_attributes)]
    career: Option<String>,

    #[cv_element_builder(text_with_attributes)]
    email: Option<String>,
    #[cv_element_builder(text_with_attributes)]
    phone: Option<String>,
    #[cv_element_builder(text_with_attributes)]
    location: Option<String>,

    // used for XML generation only
    nationality: Option<String>,
    mothertongue: Option<String>,

    #[cv_element_builder(text_with_attributes)]
    website: Option<String>,
    #[cv_element_builder(text_with_attributes)]
    github: Option<String>,
    #[cv_element_builder(text_with_attributes)]
    linkedin: Option<String>,

    image: Option<PathBuf>,
    summary: Option<SummaryElement>,
    skills: Option<SkillsElement>,
}

impl HeaderElement {
    fn try_parse_elements(header: &mut HeaderElementBuilder,
        ctx: &Context,
        root: &Path, key: &str, value: Yaml) -> Result<Option<Yaml>, String> {
        match key {
            "nationality" => header.nationality(value.einto_string()?),
            "mothertongue" => header.mothertongue(value.einto_string()?),
            "image" => {
                let image = value.einto_string()?;
                if image.is_empty() {
                    header.image(PathBuf::new())
                } else {
                    header.image(
                        root.join(image.clone())
                            .canonicalize()
                            .map_err(|e| format!("{e}: {image}"))?
                    )
                }
            },
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
                "surname" => header.add_surname(value),
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

    pub fn to_candidate_supplier_xml(&self) -> Result<XMLElement, XMLError> {
        XMLElement::new("CandidateSupplier")
            .my_add_child(
                XMLElement::new("hr:PartyID")
                    .my_add_attribute("schemeID", "Test-0001")
                    .my_add_attribute("schemeName", "PartyID")
                    .my_add_attribute("schemeAgencyName", "EUROPASS")
                    .my_add_attribute("schemeVersionID", "1.0")
            )?
            .my_add_text_child("hr:PartyName", "Owner")?
            .my_add_child(
                XMLElement::new("PersonContact")
                    .my_add_child(
                        XMLElement::new("PersonName")
                            .my_add_text_child("oa:GivenName", &self.name)?
                            .my_add_text_child("hr:FamilyName", &self.surname)?
                    )?
                    .my_add_child_if_nonempty(&self.email, |email|
                        XMLElement::new("Communication")
                            .my_add_text_child("ChannelCode", "Email")?
                            .my_add_text_child("oa:URI", email)
                    )?
            )?
            .my_add_text_child("hr:PrecedenceCode", "1")
    }

    pub fn to_candidate_person_xml(&self) -> Result<XMLElement, XMLError> {
        XMLElement::new("CandidatePerson")
            .my_add_child(
                XMLElement::new("PersonName")
                    .my_add_text_child("oa:GivenName", &self.name)?
                    .my_add_text_child("hr:FamilyName", &self.surname)?
            )?
            .my_add_child_if_nonempty(&self.email, |email|
                XMLElement::new("Communication")
                    .my_add_text_child("ChannelCode", "Email")?
                    .my_add_text_child("oa:URI", email)
            )?
            .my_add_child_if_nonempty(&self.phone, |phone| {
                let parsed = PhoneNumber::parse(phone);
                let country_dialing = parsed.as_ref().and_then(|p| p.country).map(|p| p.prefix.to_string()).unwrap_or("".to_string());
                let country_code = parsed.as_ref().and_then(|p| p.country).map(|p| p.code.to_lowercase()).unwrap_or("".to_string());
                let national_number = parsed.as_ref().map(|p| p.national_number()).unwrap_or(phone.to_string());
                XMLElement::new("Communication")
                    .my_add_text_child("ChannelCode", "Telephone")?
                    .my_add_text_child("UseCode", "mobile")?
                    .my_add_text_child_if_nonempty("CountryDialing", &country_dialing)?
                    .my_add_text_child("oa:DialNumber", &national_number)?
                    .my_add_text_child_if_nonempty("CountryCode", &country_code)
            })?
            .my_add_child_if_nonempty(&self.website, |website|
                XMLElement::new("Communication")
                    .my_add_text_child("ChannelCode", "Web")?
                    .my_add_text_child("oa:URI", &extract_markdown_uri_or_original(website))
            )?
            .my_add_child_if_nonempty(&self.github, |github|
                XMLElement::new("Communication")
                    .my_add_text_child("ChannelCode", "Social Media")?
                    .my_add_text_child("UseCode", "other")?
                    .my_add_text_child("OtherTitle", "GitHub")?
                    .my_add_text_child("oa:URI", &extract_markdown_uri_or_original(github))
            )?
            .my_add_child_if_nonempty(&self.linkedin, |linkedin|
                XMLElement::new("Communication")
                    .my_add_text_child("ChannelCode", "Social Media")?
                    .my_add_text_child("MediaUsername", &markdown_to_plaintext(linkedin))?
                    .my_add_text_child("UseCode", "linkedin")?
                    .my_add_text_child("oa:URI", &extract_markdown_uri_or_original(linkedin))
            )?
            .my_add_child_if_nonempty(&self.location, |location|
                XMLElement::new("Communication")
                    .my_add_text_child("UseCode", "home")?
                    .my_add_child(
                        XMLElement::new("Address")
                            .my_add_attribute("type", "home")
                            .my_add_text_child("oa:AddressLine", location)?
                    )
            )?
            .my_add_text_child_if_nonempty("NationalityCode", self.nationality.as_deref().unwrap_or(""))?
            .my_add_child_if_nonempty(&self.mothertongue, |mothertongue| Ok(
                build_xml_text_element("PrimaryLanguageCode", mothertongue)?
                    .my_add_attribute("name", "NORMAL")
            ))
    }

    pub fn extend_candidate_profile_xml(&self, candidate_profile: XMLElement) -> Result<XMLElement, XMLError> {
        candidate_profile
            .my_add_child_if_some(&self.summary, SummaryElement::to_experience_summary_xml)?
            .my_add_child_if_some(&self.skills, SkillsElement::to_skills_xml)?
            .my_add_child_if_some(&self.skills, SkillsElement::to_digital_skills_xml)?
            .my_add_child_if_some(&self.skills, SkillsElement::to_personal_qualifications_xml)?
            .my_add_child_if_some(&self.image.as_ref().filter(|path| !path.as_os_str().is_empty()), |image|
                XMLElement::new("eures:Attachment")
                    .my_add_text_child("oa:EmbeddedData", &BASE64_STANDARD.encode(
                        format!("data:image/png;base64,{}", BASE64_STANDARD.encode(
                            fs::read(image)
                                .map_err(|e| XMLError::IOError(e.to_string()))?
                        ))
                    ))?
                    .my_add_text_child("oa:FileType", "photo")?
                    .my_add_text_child("hr:Instructions", "ProfilePicture")
            )
    }
}

impl LatexWriter for HeaderElement {
    fn latex_write(&self, f: &mut MyWrite) -> std::io::Result<()> {
        let mut email = self.email.as_deref().unwrap_or("").to_string();
        if !email.is_empty() {
            email = format!("[{}](mailto:{})", email, email);
        }

        write_latex_command_call(
            f,
            "cv",
            &[
                &format!("{} {}", self.name, self.surname),
                self.career.as_deref().unwrap_or(""),
                self.image.as_deref().map(|p| p.to_str().unwrap()).unwrap_or(""),
                &email,
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
