//! Interfaces and utilities to produce an europass XML
//! - [Interactive schema](https://europass-unofficial.github.io/europass-schema-docs/)
//! - [PDF](https://europass.europa.eu/system/files/2020-07/europass-xml-schema-doc-v3.4.0_0.pdf)

use xml_builder::{XMLElement, XMLError};

pub const ATTACHMENT_XML_FILENAME: &str = "attachment.xml";

pub trait EuropassXmlWriter {
    fn to_europass_xml(&self) -> Result<Vec<(&'static str, XMLElement)>, XMLError>;
}

pub trait MyXmlUtils where Self : Sized {
    fn my_add_child(self, element: XMLElement) -> Result<Self, XMLError>;
    fn my_add_child_if_some<T, F: Fn(&T) -> Result<XMLElement, XMLError>>(self, s: &Option<T>, element: F) -> Result<Self, XMLError>;
    fn my_add_child_if_nonempty<F: Fn(&str) -> Result<XMLElement, XMLError>>(self, s: &Option<String>, element: F) -> Result<Self, XMLError>;
    fn my_add_text_child(self, name: &'static str, value: &str) -> Result<Self, XMLError>;
    fn my_add_text_child_if_nonempty(self, name: &'static str, value: &str) -> Result<Self, XMLError>;
    fn my_add_attribute(self, name: &'static str, value: &str) -> Self;
}

impl MyXmlUtils for XMLElement {
    fn my_add_child(mut self, element: XMLElement) -> Result<Self, XMLError> {
        self.add_child(element)?;
        Ok(self)
    }

    fn my_add_child_if_some<T, F: Fn(&T) -> Result<XMLElement, XMLError>>(mut self, s: &Option<T>, element: F) -> Result<Self, XMLError> {
        if let Some(s) = s.as_ref() {
            self.add_child(element(s)?)?;
        }
        Ok(self)
    }

    fn my_add_child_if_nonempty<F: Fn(&str) -> Result<XMLElement, XMLError>>(mut self, s: &Option<String>, element: F) -> Result<Self, XMLError> {
        if let Some(s) = s.as_deref() {
            if !s.is_empty() {
                self.add_child(element(s)?)?;
            }
        }
        Ok(self)
    }

    fn my_add_text_child(mut self, name: &'static str, value: &str) -> Result<Self, XMLError> {
        self.add_child(build_xml_text_element(name, value)?)?;
        Ok(self)
    }

    fn my_add_text_child_if_nonempty(mut self, name: &'static str, value: &str) -> Result<Self, XMLError> {
        if !value.is_empty() {
            self.add_child(build_xml_text_element(name, value)?)?;
        }
        Ok(self)
    }

    fn my_add_attribute(mut self, name: &str, value: &str) -> Self {
        self.add_attribute(name, value);
        self
    }
}

pub fn build_xml_text_element(name: &'static str, value: &str) -> Result<XMLElement, XMLError> {
    let mut element = XMLElement::new(name);
    element.add_text(escape_xml(value))?;
    Ok(element)
}

/// This is surely pwnable, but there are no other nice builder-pattern XML libraries I could find
fn escape_xml(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}
