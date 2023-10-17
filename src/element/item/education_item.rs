use crate::printers::rmarkdown::RMarkdownSectionItem;

#[derive(Debug)]
pub struct EducationItem {
    pub degree: String,
    pub institution: String,
    pub dates: String,
    pub grade: Option<String>,
    pub details: Option<String>,
}

impl RMarkdownSectionItem for EducationItem {
    fn get_field_names() -> &'static [&'static str] {
        &["degree", "institution", "dates", "grade", "details"]
    }

    fn get_fields(&self) -> Vec<String> {
        vec![
            self.degree.clone(),
            self.institution.clone(),
            self.dates.clone(),
            self.grade.clone().unwrap_or(String::new()),
            self.details.clone().unwrap_or(String::new()),
        ]
    }
}
