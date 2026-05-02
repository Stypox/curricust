use markdown::mdast::Node;

fn replace_newlines(md: &str, replacement: &'static str) -> String {
    md.replace("<br>", replacement).replace("<br/>", replacement).replace("<br />", replacement)
}

pub fn extract_markdown_uris(md: &str) -> Vec<String> {
    // calling unwrap since it can't return an error with the default settings
    let root = markdown::to_mdast(&replace_newlines(md, "\n"), &markdown::ParseOptions::default()).unwrap();

    let mut nodes = vec![root];
    let mut uris = vec![];
    while let Some(node) = nodes.pop() {
        match node {
            Node::Root(root) => nodes.extend(root.children.into_iter().rev()),
            Node::Paragraph(paragraph) => nodes.extend(paragraph.children.into_iter().rev()),
            Node::Link(link) => uris.push(link.url),
            _ => {},
        }
    }

    uris
}

pub fn extract_markdown_uri(md: &str) -> Option<String> {
    extract_markdown_uris(md).into_iter().next()
}

pub fn extract_markdown_uri_or_original(md: &str) -> String {
    if let Some(uri) = extract_markdown_uri(md) {
        uri
    } else {
        md.to_string()
    }
}

pub fn markdown_to_plaintext(md: &str) -> String {
    // calling unwrap since it can't return an error with the default settings
    let root = markdown::to_mdast(md, &markdown::ParseOptions::default()).unwrap();
    replace_newlines(&root.to_string(), "\n")
}

pub fn markdown_to_html(md: &str) -> String {
    markdown::to_html(&replace_newlines(md, "\n\n")).replace("\n", " ").trim().to_owned()
}
