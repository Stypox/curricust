use crate::util::yaml::YamlConversions;
use lazy_static::lazy_static;
use multimap::MultiMap;
use regex::{Captures, Regex};
use yaml_rust::Yaml;

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

fn into_best_matching(texts: Vec<TextWithAttributes>, attrs: &[String]) -> Option<String> {
    texts
        .into_iter()
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

pub fn into_best_matching_dictionary(
    texts: Vec<TextWithAttributes>,
    attrs: &[String],
    dictionary: &MultiMap<String, TextWithAttributes>,
) -> Result<Option<String>, String> {
    let Some(text) = into_best_matching(texts, attrs) else {
        return Ok(None);
    };

    lazy_static! {
        static ref DICTIONARY_REGEX: Regex = Regex::new(r"\{\{([^\{\}]+)\}\}").unwrap();
    }

    let mut error = None;
    let res = DICTIONARY_REGEX
        .replace_all(&text, |caps: &Captures| {
            let key = caps.get(1).map_or("", |m| m.as_str());
            if key.is_empty() {
                error = Some(String::new());
                return String::new();
            }

            let Some(values) = dictionary.get_vec(key) else {
                error = Some(key.to_string());
                return String::new();
            };

            if let Some(value) = into_best_matching(values.clone(), attrs) {
                value
            } else {
                error = Some(key.to_string());
                String::new()
            }
        })
        .to_string();

    if let Some(error) = error {
        Err(format!("Key not found in dictionary: {error}"))
    } else {
        Ok(Some(res))
    }
}
