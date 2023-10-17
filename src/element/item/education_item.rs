use crate::printers::rmarkdown::RMarkdownSectionItem;

pub struct EducationItem {}

impl RMarkdownSectionItem for EducationItem {
    fn get_field_names() -> &'static [String] {
        &[]
    }

    fn get_fields(&self) -> Vec<String> {
        vec![] // TODO use ascii::escape_default
    }
}
