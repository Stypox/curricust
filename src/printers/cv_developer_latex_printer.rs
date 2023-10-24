use super::printer::Printer;

pub trait CvDeveloperLatexPrinter {
    fn cv_developer_latex_print(&self, f: &mut Printer) -> std::io::Result<()>;
}