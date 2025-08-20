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
