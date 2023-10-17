use crate::printers::rmarkdown::RMarkdownSectionItem;

pub struct EducationItem {}

impl RMarkdownSectionItem for EducationItem {
    const N: usize = 0;

    fn get_field_names() -> [&'static str; Self::N] {
        []
    }

    fn get_fields(&self) -> [String; Self::N] {
        [] // TODO use ascii::escape_default
    }
}
