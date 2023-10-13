use derive_builder::Builder;
use yaml_rust::Yaml;

use crate::util::yaml::YamlConversions;

use super::text_with_attributes::TextWithAttributes;

#[derive(Debug, Builder)]
#[builder(pattern = "owned")]
pub struct HeaderElement {
    #[builder(setter(each(name = "add_name")))]
    name: Vec<TextWithAttributes>,
    #[builder(default, setter(each(name = "add_phone")))]
    phone: Vec<TextWithAttributes>,
}

impl HeaderElement {
    pub fn parse(mut header: HeaderElementBuilder, hash: Yaml) -> Result<HeaderElementBuilder, String> {
        let hash = hash.einto_hash()?;
        for (element_type, element_value) in hash {
            let (element_type, element_value) = TextWithAttributes::new(element_type, element_value)?;
            header = match element_type.as_str() {
                "name" => header.add_name(element_value),
                "phone" => header.add_phone(element_value),
                _ => header,
            };
        }
        Ok(header)
    }
}