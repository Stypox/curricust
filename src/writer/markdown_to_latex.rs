use lazy_static::lazy_static;
use markdown::mdast::{Heading, List, Node};
use regex::{Captures, Regex};
use std::io::Write;

use super::write::MyWrite;

fn escape_latex(value: &str) -> std::borrow::Cow<'_, str> {
    lazy_static! {
        static ref RESERVED_CHARS_REGEX: Regex = Regex::new(r"[\\\{\}\_\^\#\&\$\%\~]").unwrap();
    }

    RESERVED_CHARS_REGEX.replace_all(value, |caps: &Captures| {
        format!("\\{}", caps.get(0).unwrap().as_str())
    })
}

fn write_markdown_children(
    f: &mut MyWrite,
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
    f: &mut MyWrite,
    value: &str,
    before: &str,
    after: &str,
) -> std::io::Result<()> {
    let value = escape_latex(value);
    write!(f, "{before}{value}{after}")
}

fn write_markdown_node(f: &mut MyWrite, node: Node) -> std::io::Result<()> {
    match node {
        Node::Root(a) => write_markdown_children(f, a.children, "", ""),
        Node::List(List {
            children,
            ordered: false,
            ..
        }) => write_markdown_children(
            f,
            children,
            "{\\begin{itemize}[noitemsep,topsep=0pt,parsep=0pt,partopsep=0pt,leftmargin=0pt]\n",
            "\\end{itemize}}\n",
        ),
        Node::List(List {
            children,
            ordered: true,
            ..
        }) => write_markdown_children(
            f,
            children,
            "{\\begin{enumerate}[noitemsep,topsep=0pt,parsep=0pt,partopsep=0pt,leftmargin=0pt]\n",
            "\\end{enumerate}}\n",
        ),
        Node::ListItem(a) => write_markdown_children(f, a.children, "\\item ", "\n"),

        Node::Text(a) => write_markdown_value(f, &a.value, "", ""),
        Node::Break(_) => writeln!(f, "\\\\"),
        Node::ThematicBreak(_) => writeln!(f, "\\hrule"),
        Node::InlineCode(a) => write_markdown_value(f, &a.value, "\\texttt{", "}"),
        Node::Paragraph(a) => write_markdown_children(f, a.children, "", ""),
        Node::Html(a) => {
            if a.value == "<br>" || a.value == "<br/>" || a.value == "<br />" {
                writeln!(f, "\\\\")
            } else {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Unimplemented html element in markdown: {a:?}"),
                ));
            }
        }

        Node::Emphasis(a) => write_markdown_children(f, a.children, "\\emph{", "}"),
        Node::Strong(a) => write_markdown_children(f, a.children, "\\textbf{", "}"),
        Node::Link(a) => {
            let res = write_markdown_children(
                f,
                a.children,
                &format!("\\href{{{}}}{{", escape_latex(&a.url)),
                "}",
            );
            f.add_url_to_check(a.url);
            res
        },

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
                format!("Unimplemented element in markdown: {node:?}"),
            ))
        }
    }?;
    Ok(())
}

pub fn write_markdown(f: &mut MyWrite, md: &str) -> std::io::Result<()> {
    // calling unwrap since it can't return an error with the default settings
    let root = markdown::to_mdast(md, &markdown::ParseOptions::default()).unwrap();
    write_markdown_node(f, root)
}
