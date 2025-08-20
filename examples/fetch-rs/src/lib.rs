// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

use spin_sdk::http::{send, Request, Response};

#[allow(warnings)]
mod bindings;

use bindings::Guest;
use serde_json::Value;

struct Component;

impl Guest for Component {
    fn fetch(url: String) -> Result<String, String> {
        spin_executor::run(async move {
            let request = Request::get(url);
            let response: Response = send(request).await.map_err(|e| e.to_string())?;
            let status = response.status();
            if !(200..300).contains(status) {
                return Err(format!("Request failed with status code: {}", status));
            }
            let body = String::from_utf8_lossy(response.body());

            if let Some(content_type) = response.header("content-type").and_then(|v| v.as_str()) {
                if content_type.contains("application/json") {
                    let json: Value = serde_json::from_str(&body).map_err(|e| e.to_string())?;
                    return Ok(json_to_markdown(&json));
                } else if content_type.contains("text/html") {
                    return Ok(html_to_markdown(&body));
                }
            }

            Ok(body.into_owned())
        })
    }

    fn web_search(
        query: String,
        max_results: u32,
        language: Option<String>,
        region: Option<String>,
    ) -> Result<String, String> {
        spin_executor::run(async move {
            // Use DuckDuckGo instant answer API which is simpler and returns JSON
            let mut search_url = format!(
                "https://api.duckduckgo.com/?q={}&format=json&no_html=1&skip_disambig=1",
                urlencoding::encode(&query)
            );

            // Add language parameter if provided
            if let Some(lang) = language {
                search_url.push_str(&format!("&kl={}", urlencoding::encode(&lang)));
            }

            let request = Request::get(search_url);
            let response: Response = send(request).await.map_err(|e| e.to_string())?;
            let status = response.status();
            if !(200..300).contains(status) {
                return Err(format!(
                    "Search request failed with status code: {}",
                    status
                ));
            }

            let body = String::from_utf8_lossy(response.body());

            // Parse the JSON response from DuckDuckGo API
            let search_results = parse_duckduckgo_response(&body, max_results, &query, region);

            Ok(search_results)
        })
    }
}

fn parse_duckduckgo_response(
    json_str: &str,
    max_results: u32,
    query: &str,
    region: Option<String>,
) -> String {
    let mut results = String::new();
    results.push_str(&format!("# Web Search Results for: \"{}\"\n\n", query));
    results.push_str(&format!("**Limited to {} results**\n\n", max_results));

    if let Some(ref reg) = region {
        results.push_str(&format!("**Region:** {}\n\n", reg));
    }

    // Parse JSON response
    match serde_json::from_str::<Value>(json_str) {
        Ok(json) => {
            let mut count = 0u32;

            // Check for instant answer
            if let Some(answer) = json.get("Answer") {
                if let Some(answer_str) = answer.as_str() {
                    if !answer_str.is_empty() {
                        results.push_str("## Instant Answer\n\n");
                        results.push_str(&format!("{}\n\n", answer_str));
                        results.push_str("---\n\n");
                        count += 1;
                    }
                }
            }

            // Check for abstract
            if count < max_results {
                if let Some(abstract_text) = json.get("Abstract") {
                    if let Some(abstract_str) = abstract_text.as_str() {
                        if !abstract_str.is_empty() {
                            results.push_str("## Abstract\n\n");
                            results.push_str(&format!("{}\n\n", abstract_str));
                            if let Some(source) = json.get("AbstractSource") {
                                if let Some(source_str) = source.as_str() {
                                    results.push_str(&format!("**Source:** {}\n\n", source_str));
                                }
                            }
                            if let Some(url) = json.get("AbstractURL") {
                                if let Some(url_str) = url.as_str() {
                                    results.push_str(&format!("**URL:** {}\n\n", url_str));
                                }
                            }
                            results.push_str("---\n\n");
                            count += 1;
                        }
                    }
                }
            }

            // Check for related topics
            if count < max_results {
                if let Some(topics) = json.get("RelatedTopics") {
                    if let Some(topics_array) = topics.as_array() {
                        results.push_str("## Related Topics\n\n");
                        for (i, topic) in topics_array.iter().enumerate() {
                            if count >= max_results {
                                break;
                            }
                            if let Some(text) = topic.get("Text") {
                                if let Some(text_str) = text.as_str() {
                                    results.push_str(&format!("{}. {}\n", i + 1, text_str));
                                    if let Some(url) = topic.get("FirstURL") {
                                        if let Some(url_str) = url.as_str() {
                                            results.push_str(&format!("   **URL:** {}\n", url_str));
                                        }
                                    }
                                    results.push('\n');
                                    count += 1;
                                }
                            }
                        }
                        results.push_str("---\n\n");
                    }
                }
            }

            if count == 0 {
                results.push_str("No search results found for this query. The search engine may not have information about this topic.\n\n");
                results.push_str("**Suggestion:** Try rephrasing your search query or using more specific terms.\n\n");

                // Show raw JSON for debugging if it contains data
                if json.as_object().map_or(false, |obj| !obj.is_empty()) {
                    results.push_str("**API Response Summary:**\n");
                    if let Some(type_val) = json.get("Type") {
                        results.push_str(&format!("Type: {}\n", type_val));
                    }
                    if let Some(redirect) = json.get("Redirect") {
                        results.push_str(&format!("Redirect: {}\n", redirect));
                    }
                }
            }
        }
        Err(e) => {
            results.push_str(&format!("Error parsing search results: {}\n\n", e));
            results.push_str("**Raw response preview:**\n");
            let preview = if json_str.len() > 300 {
                &json_str[..300]
            } else {
                json_str
            };
            results.push_str(&format!("```\n{}\n```\n", preview));
        }
    }

    results
}

fn html_to_markdown(html: &str) -> String {
    let mut markdown = String::new();
    let fragment = scraper::Html::parse_fragment(html);
    let text_selector = scraper::Selector::parse("h1, h2, h3, h4, h5, h6, p, a, div").unwrap();

    for element in fragment.select(&text_selector) {
        let tag_name = element.value().name();
        let text = element
            .text()
            .collect::<Vec<_>>()
            .join(" ")
            .trim()
            .to_string();

        if text.is_empty() {
            continue;
        }

        match tag_name {
            "h1" => markdown.push_str(&format!("# {}\n\n", text)),
            "h2" => markdown.push_str(&format!("## {}\n\n", text)),
            "h3" => markdown.push_str(&format!("### {}\n\n", text)),
            "h4" => markdown.push_str(&format!("#### {}\n\n", text)),
            "h5" => markdown.push_str(&format!("##### {}\n\n", text)),
            "h6" => markdown.push_str(&format!("###### {}\n\n", text)),
            "p" => markdown.push_str(&format!("{}\n\n", text)),
            "a" => {
                if let Some(href) = element.value().attr("href") {
                    markdown.push_str(&format!("[{}]({})\n\n", text, href));
                } else {
                    markdown.push_str(&format!("{}\n\n", text));
                }
            }
            _ => markdown.push_str(&format!("{}\n\n", text)),
        }
    }

    markdown.trim().to_string()
}

fn json_to_markdown(value: &Value) -> String {
    match value {
        Value::Object(map) => {
            let mut markdown = String::new();
            for (key, val) in map {
                markdown.push_str(&format!("### {}\n\n{}\n\n", key, json_to_markdown(val)));
            }
            markdown
        }
        Value::Array(arr) => {
            let mut markdown = String::new();
            for (i, val) in arr.iter().enumerate() {
                markdown.push_str(&format!("1. {}\n", json_to_markdown(val)));
                if i < arr.len() - 1 {
                    markdown.push('\n');
                }
            }
            markdown
        }
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Null => "null".to_string(),
    }
}
bindings::export!(Component with_types_in bindings);
