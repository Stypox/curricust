use resume_cv_proc_macro::CvElementBuilder;
use std::io::Write;

use yaml_rust::Yaml;

use crate::{
    attr::text_with_attributes::TextWithAttributes,
    printers::{printer::Printer, rmarkdown::RMarkdownPrinter},
    util::yaml::YamlConversions,
};

#[derive(Debug, CvElementBuilder)]
pub struct HeaderElement {
    #[cv_element_builder(text_with_attributes)]
    name: String,
    #[cv_element_builder(text_with_attributes)]
    phone: Option<String>,
}

impl HeaderElement {
    pub fn parse(header: &mut HeaderElementBuilder, hash: Yaml) -> Result<(), String> {
        let hash = hash.einto_hash()?;
        for (key, value) in hash {
            let (key, value) = TextWithAttributes::new_yaml(key, value)?;
            match key.as_str() {
                "name" => header.add_name(value),
                "phone" => header.add_phone(value),
                _ => continue,
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
