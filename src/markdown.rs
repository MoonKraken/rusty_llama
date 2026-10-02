/// Renders Markdown to HTML that is safe to insert into the page.
///
/// Raw HTML in the source is escaped by the parser, then [ammonia] removes
/// anything that is still not a content tag. Links open in a new tab.
pub fn render(source: &str) -> String {
    let mut options = pulldown_cmark::Options::empty();
    options.insert(pulldown_cmark::Options::ENABLE_TABLES);
    options.insert(pulldown_cmark::Options::ENABLE_STRIKETHROUGH);
    // A single newline is a line break in chat. CommonMark would otherwise
    // fold it into a space, which would undo Shift+Enter in the composer.
    let parser = pulldown_cmark::Parser::new_ext(source, options).map(|event| match event {
        pulldown_cmark::Event::SoftBreak => pulldown_cmark::Event::HardBreak,
        event => event,
    });
    let mut rendered = String::new();
    pulldown_cmark::html::push_html(&mut rendered, parser);

    ammonia::Builder::new()
        .clean(&rendered)
        .to_string()
        .replace("<a href=", "<a target=\"_blank\" href=")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emphasis_lists_and_code_become_html() {
        let html = render("**bold**\n\n- one\n\n`code`");
        assert!(html.contains("<strong>bold</strong>"));
        assert!(html.contains("<li>one</li>") || html.contains("<li>\none\n</li>"));
        assert!(html.contains("<code>code</code>"));
    }

    #[test]
    fn raw_html_and_script_urls_do_not_survive() {
        let html = render("<script>alert(1)</script>\n\n[click](javascript:alert(1))");
        assert!(!html.to_ascii_lowercase().contains("<script"));
        assert!(!html.to_ascii_lowercase().contains("javascript:"));
    }

    #[test]
    fn a_single_newline_stays_a_line_break() {
        let html = render("hello\nworld");
        assert!(html.contains("hello"));
        assert!(html.contains("<br"));
        assert!(html.contains("world"));
    }

    #[test]
    fn fenced_code_keeps_its_newlines() {
        let html = render("```\nfn main() {}\n```");
        assert!(html.contains("<pre>"));
        assert!(html.contains("fn main() {}"));
        assert!(!html.contains("<br"));
    }

    #[test]
    fn links_open_in_a_new_tab() {
        let html = render("[docs](https://example.com)");
        assert!(html.contains("target=\"_blank\""));
        assert!(html.contains("rel=\"noopener noreferrer\""));
        assert!(html.contains("https://example.com"));
    }
}
