use markdown::mdast::{Heading, List, Node};
use std::io::Write;

use super::Writer;

fn write_markdown_children(
    f: &mut Writer,
    children: Vec<Node>,
    before: &str,
    after: &str,
) -> std::io::Result<()> {
    write!(f, "{before}")?;
    for child in children {
        write_markdown_node(f, child)?;
    }
    write!(f, "{after}")?;
    Ok(())
}

fn write_markdown_value(
    f: &mut Writer,
    value: &str,
    before: &str,
    after: &str,
) -> std::io::Result<()> {
    write!(f, "{before}{value}{after}")
}

fn write_markdown_node(f: &mut Writer, node: Node) -> std::io::Result<()> {
    match node {
        Node::Root(a) => write_markdown_children(f, a.children, "", ""),
        Node::List(List {
            children,
            ordered: false,
            ..
        }) => write_markdown_children(f, children, "{\\vspace{-10pt}\\begin{itemize}[noitemsep,topsep=0pt,parsep=0pt,partopsep=0pt,leftmargin=-1pt]\n", "\\end{itemize}}\n"),
        Node::List(List {
            children,
            ordered: true,
            ..
        }) => write_markdown_children(f, children, "{\\vspace{-10pt}\\begin{enumerate}[noitemsep,topsep=0pt,parsep=0pt,partopsep=0pt,leftmargin=-1pt]\n", "\\end{enumerate}}\n"),
        Node::ListItem(a) => write_markdown_children(f, a.children, "\\item ", "\n"),

        Node::Text(a) => write!(f, "{}", a.value),
        Node::Break(_) => writeln!(f, "\\\\"),
        Node::ThematicBreak(_) => writeln!(f, "\\hrule"),
        Node::InlineCode(a) => write_markdown_value(f, &a.value, "\\texttt{", "}"),
        Node::Paragraph(a) => write_markdown_children(f, a.children, "", ""),

        Node::Emphasis(a) => write_markdown_children(f, a.children, "\\emph{", "}"),
        Node::Strong(a) => write_markdown_children(f, a.children, "\\textbf{", "}"),
        Node::Link(a) => {
            write_markdown_children(f, a.children, &format!("\\href{{{}}}{{", a.url), "}")
        }

        Node::Heading(Heading {
            children, depth: 1, ..
        }) => write_markdown_children(f, children, "\\section{", "}\n"),
        Node::Heading(Heading {
            children, depth: 2, ..
        }) => write_markdown_children(f, children, "\\subsection{", "}\n"),
        Node::Heading(Heading {
            children, depth: 3, ..
        }) => write_markdown_children(f, children, "\\subsubsection{", "}\n"),
        Node::Heading(Heading {
            children, depth: 4, ..
        }) => write_markdown_children(f, children, "\\paragraph{", "}\n"),
        Node::Heading(Heading {
            children, depth: 5, ..
        }) => write_markdown_children(f, children, "\\subparagraph{", "}\n"),
        Node::Heading(Heading {
            children, depth: 6, ..
        }) => write_markdown_children(f, children, "\\textbf{", "}\n"), // like Strong

        // Node::Table(_) => todo!(),
        // Node::TableRow(_) => todo!(),
        // Node::TableCell(_) => todo!(),
        // Node::BlockQuote(_) => todo!(),
        // Node::Code(a) => todo!(),
        // Node::Delete(a) => todo!(),
        _ => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Unimplemented element in markdown: {:?}", node),
            ))
        }
    }?;
    Ok(())
}

pub fn write_markdown(f: &mut Writer, md: &str) -> std::io::Result<()> {
    // calling unwrap since it can't return an error with the default settings
    let root = markdown::to_mdast(md, &markdown::ParseOptions::default()).unwrap();
    write_markdown_node(f, root)
}
