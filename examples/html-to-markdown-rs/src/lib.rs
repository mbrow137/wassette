// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

#[allow(warnings)]
mod bindings;

use bindings::Guest;
use scraper::{Html, Selector};

struct Component;

impl Guest for Component {
    fn convert(html: String) -> Result<String, String> {
        html_to_markdown(&html).map_err(|e| e.to_string())
    }
}

/// Convert HTML to Markdown format
fn html_to_markdown(html: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut markdown = String::new();
    let fragment = Html::parse_fragment(html);

    // Process headers h1 through h6
    let h1_selector = Selector::parse("h1")?;
    let h2_selector = Selector::parse("h2")?;
    let h3_selector = Selector::parse("h3")?;
    let h4_selector = Selector::parse("h4")?;
    let h5_selector = Selector::parse("h5")?;
    let h6_selector = Selector::parse("h6")?;

    for element in fragment.select(&h1_selector) {
        let text = element
            .text()
            .collect::<Vec<_>>()
            .join(" ")
            .trim()
            .to_string();
        if !text.is_empty() {
            markdown.push_str(&format!("# {}\n\n", text));
        }
    }

    for element in fragment.select(&h2_selector) {
        let text = element
            .text()
            .collect::<Vec<_>>()
            .join(" ")
            .trim()
            .to_string();
        if !text.is_empty() {
            markdown.push_str(&format!("## {}\n\n", text));
        }
    }

    for element in fragment.select(&h3_selector) {
        let text = element
            .text()
            .collect::<Vec<_>>()
            .join(" ")
            .trim()
            .to_string();
        if !text.is_empty() {
            markdown.push_str(&format!("### {}\n\n", text));
        }
    }

    for element in fragment.select(&h4_selector) {
        let text = element
            .text()
            .collect::<Vec<_>>()
            .join(" ")
            .trim()
            .to_string();
        if !text.is_empty() {
            markdown.push_str(&format!("#### {}\n\n", text));
        }
    }

    for element in fragment.select(&h5_selector) {
        let text = element
            .text()
            .collect::<Vec<_>>()
            .join(" ")
            .trim()
            .to_string();
        if !text.is_empty() {
            markdown.push_str(&format!("##### {}\n\n", text));
        }
    }

    for element in fragment.select(&h6_selector) {
        let text = element
            .text()
            .collect::<Vec<_>>()
            .join(" ")
            .trim()
            .to_string();
        if !text.is_empty() {
            markdown.push_str(&format!("###### {}\n\n", text));
        }
    }

    // Process paragraphs
    let p_selector = Selector::parse("p")?;
    for element in fragment.select(&p_selector) {
        let text = element
            .text()
            .collect::<Vec<_>>()
            .join(" ")
            .trim()
            .to_string();
        if !text.is_empty() {
            markdown.push_str(&format!("{}\n\n", text));
        }
    }

    // Process links
    let a_selector = Selector::parse("a")?;
    for element in fragment.select(&a_selector) {
        let text = element
            .text()
            .collect::<Vec<_>>()
            .join(" ")
            .trim()
            .to_string();
        if !text.is_empty() {
            if let Some(href) = element.value().attr("href") {
                markdown.push_str(&format!("[{}]({})\n\n", text, href));
            } else {
                markdown.push_str(&format!("{}\n\n", text));
            }
        }
    }

    // Process unordered lists
    let ul_selector = Selector::parse("ul")?;
    let li_selector = Selector::parse("li")?;
    for ul_element in fragment.select(&ul_selector) {
        for li_element in ul_element.select(&li_selector) {
            let text = li_element
                .text()
                .collect::<Vec<_>>()
                .join(" ")
                .trim()
                .to_string();
            if !text.is_empty() {
                markdown.push_str(&format!("- {}\n", text));
            }
        }
        markdown.push('\n');
    }

    // Process ordered lists
    let ol_selector = Selector::parse("ol")?;
    for ol_element in fragment.select(&ol_selector) {
        let mut index = 1;
        for li_element in ol_element.select(&li_selector) {
            let text = li_element
                .text()
                .collect::<Vec<_>>()
                .join(" ")
                .trim()
                .to_string();
            if !text.is_empty() {
                markdown.push_str(&format!("{}. {}\n", index, text));
                index += 1;
            }
        }
        markdown.push('\n');
    }

    // Process blockquotes
    let blockquote_selector = Selector::parse("blockquote")?;
    for element in fragment.select(&blockquote_selector) {
        let text = element
            .text()
            .collect::<Vec<_>>()
            .join(" ")
            .trim()
            .to_string();
        if !text.is_empty() {
            markdown.push_str(&format!("> {}\n\n", text));
        }
    }

    // Process code blocks
    let pre_selector = Selector::parse("pre")?;
    for element in fragment.select(&pre_selector) {
        let text = element.text().collect::<Vec<_>>().join("\n");
        if !text.trim().is_empty() {
            markdown.push_str(&format!("```\n{}\n```\n\n", text.trim()));
        }
    }

    // Process inline code
    let code_selector = Selector::parse("code")?;
    for element in fragment.select(&code_selector) {
        let text = element
            .text()
            .collect::<Vec<_>>()
            .join(" ")
            .trim()
            .to_string();
        if !text.is_empty() {
            markdown.push_str(&format!("`{}`\n\n", text));
        }
    }

    // Process emphasis
    let em_selector = Selector::parse("em, i")?;
    for element in fragment.select(&em_selector) {
        let text = element
            .text()
            .collect::<Vec<_>>()
            .join(" ")
            .trim()
            .to_string();
        if !text.is_empty() {
            markdown.push_str(&format!("*{}*\n\n", text));
        }
    }

    // Process strong
    let strong_selector = Selector::parse("strong, b")?;
    for element in fragment.select(&strong_selector) {
        let text = element
            .text()
            .collect::<Vec<_>>()
            .join(" ")
            .trim()
            .to_string();
        if !text.is_empty() {
            markdown.push_str(&format!("**{}**\n\n", text));
        }
    }

    // Process divs and other generic containers
    let div_selector = Selector::parse("div, span")?;
    for element in fragment.select(&div_selector) {
        let text = element
            .text()
            .collect::<Vec<_>>()
            .join(" ")
            .trim()
            .to_string();
        if !text.is_empty() {
            markdown.push_str(&format!("{}\n\n", text));
        }
    }

    Ok(markdown.trim().to_string())
}

bindings::export!(Component with_types_in bindings);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_headers() {
        let html = r#"
            <h1>Header 1</h1>
            <h2>Header 2</h2>
            <h3>Header 3</h3>
            <h4>Header 4</h4>
            <h5>Header 5</h5>
            <h6>Header 6</h6>
        "#;
        let result = html_to_markdown(html).unwrap();
        assert!(result.contains("# Header 1"));
        assert!(result.contains("## Header 2"));
        assert!(result.contains("### Header 3"));
        assert!(result.contains("#### Header 4"));
        assert!(result.contains("##### Header 5"));
        assert!(result.contains("###### Header 6"));
    }

    #[test]
    fn test_paragraphs() {
        let html = r#"
            <p>This is a paragraph.</p>
            <p>This is another paragraph.</p>
        "#;
        let result = html_to_markdown(html).unwrap();
        assert!(result.contains("This is a paragraph."));
        assert!(result.contains("This is another paragraph."));
    }

    #[test]
    fn test_emphasis_and_strong() {
        let html = r#"
            <p>This is <em>emphasis</em> and <strong>strong</strong>.</p>
            <p>This is <i>italic</i> and <b>bold</b>.</p>
        "#;
        let result = html_to_markdown(html).unwrap();
        assert!(result.contains("*emphasis*"));
        assert!(result.contains("**strong**"));
        assert!(result.contains("*italic*"));
        assert!(result.contains("**bold**"));
    }

    #[test]
    fn test_links() {
        let html = r#"<a href="https://example.com">Example Link</a>"#;
        let result = html_to_markdown(html).unwrap();
        assert!(result.contains("[Example Link](https://example.com)"));
    }

    #[test]
    fn test_link_without_href() {
        let html = r#"<a>Plain Link Text</a>"#;
        let result = html_to_markdown(html).unwrap();
        assert!(result.contains("Plain Link Text"));
        assert!(!result.contains("["));
        assert!(!result.contains("]"));
    }

    #[test]
    fn test_unordered_list() {
        let html = r#"
            <ul>
                <li>Item 1</li>
                <li>Item 2</li>
                <li>Item 3</li>
            </ul>
        "#;
        let result = html_to_markdown(html).unwrap();
        assert!(result.contains("- Item 1"));
        assert!(result.contains("- Item 2"));
        assert!(result.contains("- Item 3"));
    }

    #[test]
    fn test_ordered_list() {
        let html = r#"
            <ol>
                <li>First item</li>
                <li>Second item</li>
                <li>Third item</li>
            </ol>
        "#;
        let result = html_to_markdown(html).unwrap();
        assert!(result.contains("1. First item"));
        assert!(result.contains("2. Second item"));
        assert!(result.contains("3. Third item"));
    }

    #[test]
    fn test_blockquotes() {
        let html = r#"<blockquote>This is a blockquote.</blockquote>"#;
        let result = html_to_markdown(html).unwrap();
        assert!(result.contains("> This is a blockquote."));
    }

    #[test]
    fn test_code_blocks() {
        let html = r#"<pre>let x = 5;
let y = 10;</pre>"#;
        let result = html_to_markdown(html).unwrap();
        assert!(result.contains("```"));
        assert!(result.contains("let x = 5;"));
        assert!(result.contains("let y = 10;"));
    }

    #[test]
    fn test_inline_code() {
        let html = r#"<p>Use the <code>println!</code> macro.</p>"#;
        let result = html_to_markdown(html).unwrap();
        assert!(result.contains("`println!`"));
    }

    #[test]
    fn test_empty_input() {
        let html = "";
        let result = html_to_markdown(html).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_whitespace_only() {
        let html = "   \n\t  ";
        let result = html_to_markdown(html).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_complex_html() {
        let html = r#"
            <h1>Hello World</h1>
            <p>This is a <strong>paragraph</strong> with <em>emphasis</em>.</p>
            <ul>
                <li>Item 1</li>
                <li>Item 2</li>
            </ul>
            <blockquote>This is a quote.</blockquote>
            <pre>code block</pre>
        "#;
        let result = html_to_markdown(html).unwrap();
        
        assert!(result.contains("# Hello World"));
        assert!(result.contains("**paragraph**"));
        assert!(result.contains("*emphasis*"));
        assert!(result.contains("- Item 1"));
        assert!(result.contains("- Item 2"));
        assert!(result.contains("> This is a quote."));
        assert!(result.contains("```"));
        assert!(result.contains("code block"));
    }

    #[test]
    fn test_nested_elements() {
        let html = r#"
            <div>
                <p>Paragraph in div</p>
                <span>Span text</span>
            </div>
        "#;
        let result = html_to_markdown(html).unwrap();
        assert!(result.contains("Paragraph in div"));
        assert!(result.contains("Span text"));
    }

    #[test]
    fn test_guest_interface() {
        let html = r#"<h1>Test</h1><p>Content</p>"#.to_string();
        let result = Component::convert(html).unwrap();
        assert!(result.contains("# Test"));
        assert!(result.contains("Content"));
    }

    #[test]
    fn test_guest_interface_with_error() {
        // Test with malformed selector (this shouldn't actually error with scraper, but tests the error path)
        let html = "".to_string();
        let result = Component::convert(html);
        assert!(result.is_ok());
    }
}
