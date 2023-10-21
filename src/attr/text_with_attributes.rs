use crate::util::yaml::YamlConversions;
use multimap::MultiMap;
use yaml_rust::Yaml;
use regex::{Regex, Captures};
use lazy_static::lazy_static;

#[derive(Debug, Clone)]
pub struct TextWithAttributes {
    text: String,
    attrs: Vec<String>,
}

impl TextWithAttributes {
    pub fn new_yaml(key: Yaml, value: Yaml) -> Result<(String, TextWithAttributes), String> {
        Self::new_string(key.einto_string()?, value)
    }

    pub fn new_string(key: String, value: Yaml) -> Result<(String, TextWithAttributes), String> {
        let value = value.einto_string()?;
        let mut key = key.split('-');

        let name = key
            .next()
            .expect("split() produces at least one value")
            .to_string();
        let attributes = key.map(|a| a.to_string()).collect();
        Ok((
            name,
            TextWithAttributes {
                text: value,
                attrs: attributes,
            },
        ))
    }

    fn intersection_cardinality(&self, other_attrs: &[String]) -> usize {
        return self
            .attrs
            .iter()
            .filter(|e| other_attrs.contains(e))
            .count();
    }
}

pub trait TextWithAttributesCollection {
    fn into_best_matching(self, attrs: &[String]) -> Option<String>;
    fn into_best_matching_dictionary(self, attrs: &[String], dictionary: &MultiMap<String, TextWithAttributes>) -> Result<Option<String>, String>;
}

impl TextWithAttributesCollection for Vec<TextWithAttributes> {
    fn into_best_matching(self, attrs: &[String]) -> Option<String> {
        self.into_iter()
            // max_by prefers the last element in case there are equal matches
            .max_by(|a, b| {
                let card_a = a.intersection_cardinality(attrs);
                let card_b = b.intersection_cardinality(attrs);

                if card_a == card_b {
                    // in case of parity, prefer items with less attributes overall
                    b.attrs.len().cmp(&a.attrs.len())
                } else {
                    card_a.cmp(&card_b)
                }
            })
            .map(|e| e.text)
    }

    fn into_best_matching_dictionary(self, attrs: &[String], dictionary: &MultiMap<String, TextWithAttributes>) -> Result<Option<String>, String> {
        let Some(text) = Self::into_best_matching(self, attrs) else {
            return Ok(None);
        };
    
        lazy_static! {
            static ref DICTIONARY_REGEX: Regex = Regex::new("\\{\\{([^\\{\\}]+)\\}\\}").unwrap();
        }

        let mut error = None;
        let res = DICTIONARY_REGEX.replace_all(&text, |caps: &Captures| {
            let key = caps.get(1).map_or("", |m| m.as_str());
            if key.is_empty() {
                error = Some(String::new());
                return String::new();
            }

            let Some(values) = dictionary.get_vec(key) else {
                error = Some(key.to_string());
                return String::new();
            };
            
            if let Some(value) = values.clone().into_best_matching(attrs) {
                value
            } else {
                error = Some(key.to_string());
                return String::new();
            }
        });

        if let Some(error) = error {
            Err(format!("Key not found in dictionary: {error}"))
        } else {
            Ok(Some(res.to_string()))
        }
    }
}
