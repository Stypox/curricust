use std::collections::HashMap;

use multimap::MultiMap;
use variant_count::VariantCount;

use crate::attr::text_with_attributes::TextWithAttributes;

#[derive(PartialEq, Eq, Hash, Clone, Copy, VariantCount)]
pub enum AttributeType {
    Locale,
    Display,
}

#[derive(Default, Clone)]
struct AttributeData<T>
where
    T: Clone,
{
    value: T,
    overrides: HashMap<String, T>,
}

#[derive(Clone)]
pub struct Context {
    attrs: [AttributeData<Option<String>>; AttributeType::VARIANT_COUNT],
    order: AttributeData<Option<i64>>,
    visibility: AttributeData<bool>,
    pub dictionary: MultiMap<String, TextWithAttributes>,
}

impl Default for Context {
    fn default() -> Self {
        Self { attrs: Default::default(), order: Default::default(), visibility: AttributeData { value: true, overrides: Default::default() }, dictionary: Default::default() }
    }
}

impl Context {
    pub fn get_active_attrs(&self, id: &Option<String>) -> Vec<String> {
        self.attrs
            .clone()
            .into_iter()
            .filter_map(|e| {
                if let Some(id) = &id {
                    if let Some(res) = e.overrides.get(id) {
                        return res.clone();
                    }
                }
                e.value
            })
            .collect()
    }

    pub fn get_order(&self, id: &Option<String>) -> i64 {
        if let Some(id) = id {
            if let Some(order) = self.order.overrides.get(id) {
                return order.unwrap_or(i64::MAX);
            }
        }
        self.order.value.unwrap_or(i64::MAX)
    }

    pub fn get_visibility(&self, id: &Option<String>) -> bool {
        if let Some(id) = id {
            if let Some(visibility) = self.visibility.overrides.get(id) {
                return *visibility;
            }
        }
        self.visibility.value
    }

    pub fn set_attr(&mut self, attr_type: AttributeType, value: Option<String>) {
        let data = &mut self.attrs[attr_type as usize];
        data.value = value;
        data.overrides.clear();
    }

    pub fn override_attr(&mut self, id: String, attr_type: AttributeType, value: Option<String>) {
        self.attrs[attr_type as usize].overrides.insert(id, value);
    }

    pub fn set_order(&mut self, pos: Option<i64>) {
        self.order.value = pos;
    }

    pub fn override_order(&mut self, id: String, pos: Option<i64>) {
        self.order.overrides.insert(id, pos);
    }

    pub fn set_visibility(&mut self, visible: bool) {
        self.visibility.value = visible;
    }

    pub fn override_visibility(&mut self, id: String, visible: bool) {
        self.visibility.overrides.insert(id, visible);
    }
}
