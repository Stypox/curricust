use std::io::Write;

use yaml_rust::Yaml;

use crate::{
    printers::{printer::Printer, rmarkdown::RMarkdownPrinter},
    util::yaml::YamlConversions,
};

use super::text_with_attributes::{TextWithAttributes, TextWithAttributesCollection};

#[derive(Debug)]
pub struct HeaderElement {
    name: String,
    phone: Option<String>,
}

#[derive(Default)]
pub struct HeaderElementBuilder {
    name: Vec<TextWithAttributes>,
    phone: Vec<TextWithAttributes>,
}

impl HeaderElementBuilder {
    fn add_name(&mut self, e: TextWithAttributes) {
        self.name.push(e);
    }
    fn add_phone(&mut self, e: TextWithAttributes) {
        self.name.push(e);
    }
    pub fn build(self, active_attrs: &[String]) -> Result<HeaderElement, String> {
        Ok(HeaderElement {
            name: self
                .name
                .into_best_matching(active_attrs)
                .ok_or("Missing name in header".to_string())?,
            phone: self.phone.into_best_matching(active_attrs),
        })
    }
}

impl HeaderElement {
    pub fn parse(header: &mut HeaderElementBuilder, hash: Yaml) -> Result<(), String> {
        let hash = hash.einto_hash()?;
        for (element_type, element_value) in hash {
            let (element_type, element_value) =
                TextWithAttributes::new(element_type, element_value)?;
            match element_type.as_str() {
                "name" => header.add_name(element_value),
                "phone" => header.add_phone(element_value),
                _ => {}
            };
        }
        Ok(())
    }
}

impl RMarkdownPrinter for HeaderElement {
    fn rmarkdown_print(&self, f: &mut Printer) -> std::io::Result<()> {
        writeln!(f, "name: {:?}", self.name)?;
        if let Some(phone) = &self.phone {
            writeln!(f, "phone: {phone:?}")?;
        }
        Ok(())
    }
}
