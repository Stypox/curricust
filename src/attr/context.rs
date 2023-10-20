use std::collections::HashMap;

use multimap::MultiMap;

use crate::attr::text_with_attributes::TextWithAttributes;

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum AttributeType {
    Locale,
    Display,
}

#[derive(Default, Clone)]
struct AttributeData {
    value: Option<String>,
    overrides: HashMap<String, Option<String>>,
}

#[derive(Default)]
pub struct Context {
    attrs: [AttributeData; std::mem::variant_count::<AttributeType>()],
    pub dictionary: MultiMap<String, TextWithAttributes>,
}

impl Context {
    pub fn get_active_attrs(&self, id: Option<String>) -> Vec<String> {
        self.attrs.clone().into_iter()
            .filter_map(|e| {
                if let Some(id) = &id {
                    if let Some(res) = e.overrides.get(id) {
                        return res.clone();
                    }
                }
                return e.value;
            })
            .collect()
    }

    pub fn set_attr(&mut self, attr_type: AttributeType, value: Option<String>) {
        let data = &mut self.attrs[attr_type as usize];
        data.value = value;
        data.overrides.clear();
    }

    pub fn append_override(&mut self, id: String, attr_type: AttributeType, value: Option<String>) {
        self.attrs[attr_type as usize].overrides.insert(id, value);
    }
}