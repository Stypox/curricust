use resume_cv_proc_macro::CvElementBuilder;
use std::io::Write;

use yaml_rust::Yaml;

use crate::{
    attr::text_with_attributes::TextWithAttributes,
    printers::{Printer, rmarkdown::RMarkdownPrinter, cv_developer_latex_printer::CvDeveloperLatexPrinter},
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
    website_href: Option<String>,
    #[cv_element_builder(text_with_attributes)]
    github: Option<String>,
    #[cv_element_builder(text_with_attributes)]
    github_href: Option<String>,
    #[cv_element_builder(text_with_attributes)]
    linkedin: Option<String>,
    #[cv_element_builder(text_with_attributes)]
    linkedin_href: Option<String>,
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

#[allow(clippy::write_literal)]
impl CvDeveloperLatexPrinter for HeaderElement {
    fn cvdl_print(&self, f: &mut Printer) -> std::io::Result<()> {
        writeln!(f, "{}", r#"\begin{minipage}[t]{0.5\textwidth}"#)?;
        writeln!(f, "{}", r#"    \vspace{-\baselineskip}"#)?;
        writeln!(f, "{}{}{}", r#"    { \fontsize{16}{20} \textcolor{black}{\textbf{\MakeUppercase{"#, self.name, r#"}}}}"#)?;
        if let Some(career) = &self.career {
            writeln!(f, "{}", r#"    \vspace{6pt}"#)?;
            writeln!(f, "{}{career}{}", r#"    {\Large "#, r#"}"#)?;
        };
        writeln!(f, "{}", r#"\end{minipage}"#)?;

        fn icon(f: &mut Printer, icon_name: &str, content: &str, href: &Option<String>) -> std::io::Result<()> {
            write!(f, "{}{icon_name}{}", r#"    \icon{"#, r#"}{11}{"#)?;
            if let Some(href) = href {
                write!(f, "{}{href}{}{content}{}", r#"\href{"#, r#"}{"#, r#"}"#)?;
            } else {
                write!(f, "{content}")?;
            }
            writeln!(f, "{}", r#"}\\"#)?;
            Ok(())
        }

        fn maybe_icon(f: &mut Printer, icon_name: &str, content: &Option<String>, href: &Option<String>) -> std::io::Result<()> {
            if let Some(content) = content {
                icon(f, icon_name, content, href)
            } else {
                Ok(())
            }
        }

        writeln!(f, "{}", r#"\hfill"#)?;
        writeln!(f, "{}", r#"\begin{minipage}[t]{0.2\textwidth}"#)?;
        writeln!(f, "{}", r#"    \vspace{-\baselineskip}"#)?;
        if let Some(email) = &self.email {
            icon(f, "Envelope", email, &Some("mailto:".to_string() + email))?;
        }
        maybe_icon(f, "Phone", &self.phone, &None)?;
        maybe_icon(f, "MapMarker", &self.location, &None)?;
        writeln!(f, "{}", r#"\end{minipage}"#)?;

        writeln!(f, "{}", r#"\begin{minipage}[t]{0.27\textwidth}"#)?;
        writeln!(f, "{}", r#"    \vspace{-\baselineskip}"#)?;
        maybe_icon(f, "Globe", &self.website, &self.website_href)?;
        maybe_icon(f, "Github", &self.github, &self.github_href)?;
        maybe_icon(f, "LinkedinSquare", &self.linkedin, &self.linkedin_href)?;
        writeln!(f, "{}", r#"\end{minipage}"#)?;
        Ok(())
    }
}