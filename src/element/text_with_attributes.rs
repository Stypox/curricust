use crate::util::yaml::YamlConversions;
use yaml_rust::Yaml;

#[derive(Debug)]
pub struct TextWithAttributes {
    text: String,
    attrs: Vec<String>,
}

impl TextWithAttributes {
    pub fn new(key: Yaml, value: Yaml) -> Result<(String, TextWithAttributes), String> {
        let key = key.einto_string()?;
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
}

impl TextWithAttributesCollection for Vec<TextWithAttributes> {
    fn into_best_matching(self, attrs: &[String]) -> Option<String> {
        self
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
}
