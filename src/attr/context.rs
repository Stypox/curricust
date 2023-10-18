use std::collections::HashMap;

use multimap::MultiMap;

use crate::attr::text_with_attributes::TextWithAttributes;

#[derive(PartialEq, Eq, Hash)]
pub enum AttributeType {
    Locale,
    Display,
}

#[derive(Default)]
pub struct Context {
    attrs: HashMap<AttributeType, String>,
    pub dictionary: MultiMap<String, TextWithAttributes>,
    overrides: HashMap<String, (AttributeType, String)>
}

impl Context {
    pub fn get_active_attrs(&self, id: Option<String>) -> Vec<String> {
        self.attrs.values().cloned().collect()
    }

    pub fn set_attr(&mut self, key: AttributeType, value: String) {
        self.attrs.insert(key, value);
    }
}