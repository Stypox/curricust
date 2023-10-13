use std::{fs::{self}, fmt::Display, env};

use multimap::MultiMap;
use yaml_rust::{YamlLoader, Yaml, yaml::Hash};

extern crate yaml_rust;

trait ErrorToString<T> {
    fn err_str(self) -> Result<T, String>;
}

impl<T, E> ErrorToString<T> for Result<T, E> where E: Display {
    fn err_str(self) -> Result<T, String> {
        self.map_err(|e| e.to_string())
    }
}


trait YamlConversions {
    fn einto_string(self) -> Result<String, String>;
    fn einto_hash(self) -> Result<Hash, String>;
    fn einto_vec(self) -> Result<Vec<Yaml>, String>;
    fn einto_single_element_hash(self) -> Result<(String, Yaml), String>;
}

impl YamlConversions for Yaml {
    fn einto_string(self) -> Result<String, String> {
        match self {
            Yaml::Real(a) => Ok(a),
            Yaml::Integer(a) => Ok(a.to_string()),
            Yaml::String(a) => Ok(a),
            Yaml::Boolean(a) => Ok(a.to_string()),
            Yaml::Array(a) => Err(format!("Unexpected array: {a:?}")),
            Yaml::Hash(a) => Err(format!("Unexpected map: {a:?}")),
            Yaml::Alias(a) => Err(format!("Unexpected alias: {a:?}")),
            Yaml::Null => Err("Unexpected null".to_string()),
            Yaml::BadValue => Err("Unexpected bad value".to_string()),
        }
    }

    fn einto_hash(self) -> Result<Hash, String> {
        self.into_hash().ok_or("Expected hash".to_string())
    }

    fn einto_vec(self) -> Result<Vec<Yaml>, String> {
        self.into_vec().ok_or("Expected arrat".to_string())
    }

    fn einto_single_element_hash(self) -> Result<(String, Yaml), String> {
        let hash = self.einto_hash()?;
        let mut hash_iter = hash.into_iter();

        let (element_type, element_value) = hash_iter.next()
            .ok_or("Unexpected empty map".to_string())?;
        let element_type = element_type.einto_string()?;

        if let Some(v) = hash_iter.next() {
            Err(format!("Unexpected sibiling of element type {element_type:?}: {v:?}"))
        } else {
            Ok((element_type, element_value))
        }
    }
}



#[derive(Debug)]
struct TextWithAttributes {
    text: String,
    attrs: Vec<String>,
}

impl TextWithAttributes {
    fn new(key: Yaml, value: Yaml) -> Result<(String, TextWithAttributes), String> {
        let key = key.einto_string()?;
        let value = value.einto_string()?;
        let mut key = key.split('-');

        let name = key.next().expect("split() produces at least one value").to_string();
        let attributes = key.map(|a| a.to_string()).collect();
        Ok((name, TextWithAttributes { text: value, attrs: attributes }))
    }
}

#[derive(Debug)]
struct BaseElement {
    locale: String,
    dictionary: MultiMap<String, TextWithAttributes>,
}

impl BaseElement {
    fn parse_dictionary(dictionary: &mut MultiMap<String, TextWithAttributes>, hash: Yaml) -> Result<(), String> {
        let hash = hash.einto_hash()?;
        for (key, value) in hash.into_iter() {
            let (key, value) = TextWithAttributes::new(key, value)?;
            dictionary.insert(key, value);
        }
        Ok(())
    }

    fn new(array: Yaml) -> Result<BaseElement, String> {
        let array = array.einto_vec()?;
        let mut locale = None;
        let mut dictionary = MultiMap::new();

        for yaml in array {
            let (element_type, element_value) = yaml.einto_single_element_hash()?;

            match element_type.as_str() {
                "locale" => locale = Some(element_value.einto_string()?),
                "dictionary" => Self::parse_dictionary(&mut dictionary, element_value)?,
                _ => {}//return Err(format!("Base element can't have children of type {element_type:?}")),
            }
        }

        let locale = locale.ok_or("Did not find locale in base element")?;
        Ok(BaseElement { locale, dictionary })
    }
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let usage = || { format!("Usage: {} filename.yml", args.get(0).unwrap_or(&"resume-cv-rust".to_string())) };
    if args.len() != 2 {
        return Err(usage());
    }
    let filename = args.get(1).ok_or_else(usage)?;

    let file = fs::read_to_string(filename).err_str()?;
    let deserialized_map = YamlLoader::load_from_str(file.as_str()).err_str()?;
    println!("{deserialized_map:?}\n");
    if let Some(array) = deserialized_map.into_iter().next() {
        let base_element = BaseElement::new(array)?;
        println!("Base element: {base_element:?}");
    }
    Ok(())
}
