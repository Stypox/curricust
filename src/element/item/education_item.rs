use resume_cv_proc_macro::{CvElementBuilder, CvRMarkdownItem};

use yaml_rust::yaml::Hash;

use crate::{attr::{context::Context, text_with_attributes::TextWithAttributes}, util::yaml::YamlConversions};

use super::SectionItem;

#[derive(Debug, CvElementBuilder, CvRMarkdownItem)]
pub struct EducationItem {
    #[cv_element_builder(text_with_attributes)]
    pub degree: String,
    #[cv_element_builder(text_with_attributes)]
    pub institution: String,
    #[cv_element_builder(text_with_attributes)]
    pub dates: String,
    #[cv_element_builder(text_with_attributes)]
    pub grade: Option<String>,
    #[cv_element_builder(text_with_attributes)]
    pub details: Option<String>,
}

impl SectionItem for EducationItem {
    fn parse(hash: Hash, ctx: &Context) -> Result<Self, String> {
        let mut builder = EducationItem::builder();
        
        for (key, value) in hash {
            let key = key.einto_string()?;
            if key == "id" {
                builder.id(value.einto_string()?);
                continue;
            }
            let (key, value) = TextWithAttributes::new_string(key, value)?;

            match key.as_str() {
                "degree" => builder.add_degree(value),
                "institution" => builder.add_institution(value),
                "dates" => builder.add_dates(value),
                "grade" => builder.add_grade(value),
                "details" => builder.add_details(value),
                _ => return Err(format!("Unknown key in section item {key}")),
            };
        }

        builder.build(ctx)
    }
}