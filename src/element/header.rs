use resume_cv_proc_macro::CvElementBuilder;
use std::io::Write;

use yaml_rust::Yaml;

use crate::{
    attr::text_with_attributes::TextWithAttributes,
    printers::{
        latex_printer::LatexPrinter, Writer, markdown_to_latex::write_markdown,
    },
    util::yaml::YamlConversions,
};

#[derive(Debug, CvElementBuilder)]
pub struct HeaderElement {
    #[cv_element_builder(text_with_attributes)]
    name: String,
    #[cv_element_builder(text_with_attributes)]
    career: Option<String>,

    #[cv_element_builder(text_with_attributes)]
    email: Option<String>,
    #[cv_element_builder(text_with_attributes)]
    phone: Option<String>,
    #[cv_element_builder(text_with_attributes)]
    location: Option<String>,

    #[cv_element_builder(text_with_attributes)]
    website: Option<String>,
    #[cv_element_builder(text_with_attributes)]
    github: Option<String>,
    #[cv_element_builder(text_with_attributes)]
    linkedin: Option<String>,
}

impl HeaderElement {
    pub fn parse(header: &mut HeaderElementBuilder, hash: Yaml) -> Result<(), String> {
        let hash = hash.einto_hash()?;
        for (key, value) in hash {
            let (key, value) = TextWithAttributes::new_yaml(key, value)?;
            match key.as_str() {
                "name" => header.add_name(value),
                "career" => header.add_career(value),
                "email" => header.add_email(value),
                "phone" => header.add_phone(value),
                "location" => header.add_location(value),
                "website" => header.add_website(value),
                "github" => header.add_github(value),
                "linkedin" => header.add_linkedin(value),
                _ => return Err(format!("Unknown key in header: {key}")),
            };
        }
        Ok(())
    }
}

#[allow(clippy::write_literal)]
impl LatexPrinter for HeaderElement {
    fn cvdl_print(&self, f: &mut Writer) -> std::io::Result<()> {
        fn maybe_icon(
            f: &mut Writer,
            icon_name: &str,
            content: &Option<String>,
        ) -> std::io::Result<()> {
            if let Some(content) = content {
                if !content.is_empty() {
                    write!(f, "{}{icon_name}{}", r#"    \icon{"#, r#"}{11}{"#)?;
                    write_markdown(f, content)?;
                    writeln!(f, "{}", r#"}\\"#)?;
                }
            }
            Ok(())
        }

        writeln!(f, "{}", r#"\begin{minipage}[t]{0.3\textwidth}"#)?;
        writeln!(f, "{}", r#"    \vspace{-\baselineskip}"#)?;
        write!(
            f,
            "{}",
            r#"    {\fontsize{16}{20}\textbf{\MakeUppercase{"#
        )?;
        write_markdown(f, &self.name)?;
        writeln!(f, "{}", r#"}}}"#)?;

        if let Some(career) = &self.career {
            write!(f, "{}\n{}", r#"    \vspace{6pt}\\"#, r#"    {\Large "#)?;
            write_markdown(f, career)?;
            writeln!(f, "{}", r#"}"#)?;
        };
        writeln!(f, "{}", r#"\end{minipage}"#)?;

        writeln!(f, "{}", r#"\hfill"#)?;
        writeln!(f, "{}", r#"\begin{minipage}[t]{0.35\textwidth}"#)?;
        writeln!(f, "{}", r#"    \vspace{-\baselineskip}"#)?;
        maybe_icon(f, "Envelope", &self.email)?;
        maybe_icon(f, "Phone", &self.phone)?;
        maybe_icon(f, "MapMarker", &self.location)?;
        writeln!(f, "{}", r#"\end{minipage}"#)?;

        writeln!(f, "{}", r#"\begin{minipage}[t]{0.25\textwidth}"#)?;
        writeln!(f, "{}", r#"    \vspace{-\baselineskip}"#)?;
        maybe_icon(f, "Globe", &self.website)?;
        maybe_icon(f, "Github", &self.github)?;
        maybe_icon(f, "LinkedinSquare", &self.linkedin)?;
        writeln!(f, "{}", r#"\end{minipage}"#)?;
        Ok(())
    }
}
