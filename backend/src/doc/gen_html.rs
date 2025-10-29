use base64::Engine;
use comrak::plugins::syntect::SyntectAdapterBuilder;
use comrak::{Plugins, markdown_to_html_with_plugins};
use lol_html::{HtmlRewriter, Settings, element};
// Taken from https://github.com/sindresorhus/github-markdown-css/blob/bedb4b771f5fa1ae117df597c79993fd1eb4dff0/github-markdown-light.css
pub const GITHUB_MARKDOWN_LIGHT_CSS: &str =
    include_str!("../../resources/github-markdown-light.css");
pub const PAGE_BREAK_AND_CENTER_CSS: &str =
    include_str!("../../resources/page_break_and_center.css");

pub fn embed_local_images(html: &str) -> String {
    let mut output = vec![];
    let mut rewriter = HtmlRewriter::new(
        Settings {
            element_content_handlers: vec![
                // Replace local image sources with base64-encoded data URIs
                element!("img[src]", |el| {
                    let img_src = el.get_attribute("src");

                    if let Some(src) = img_src.as_ref() {
                        // check if it's a local file, i.e., starts with "file://"
                        if let Some(path) = src.strip_prefix("file://") {
                            // read file from path
                            let img_data = std::fs::read(path).unwrap_or_default();
                            let mime_type = mime_guess::from_path(path)
                                .first_or_octet_stream()
                                .essence_str()
                                .to_string();
                            let base64_data = base64::engine::GeneralPurpose::new(
                                &base64::alphabet::STANDARD,
                                base64::engine::general_purpose::PAD,
                            )
                            .encode(&img_data);
                            let data_uri = format!("data:{};base64,{}", mime_type, base64_data);
                            _ = el.set_attribute("src", &data_uri);
                        }
                    };

                    Ok(())
                }),
            ],
            ..Settings::default()
        },
        |c: &[u8]| output.extend_from_slice(c),
    );

    rewriter.write(html.as_bytes()).unwrap();
    rewriter.end().unwrap();
    String::from_utf8(output).unwrap()
}

pub fn markdown_to_html(input: &str) -> String {
    let fragment = markdown_to_html_fragment(input);
    let html = wrap_in_html_doc(
        &fragment,
        "Document", // TODO get title from arg or extract from markdown
        &[GITHUB_MARKDOWN_LIGHT_CSS.to_string()],
    );

    embed_local_images(&html)
}

// TODO: Make all options configurable
pub fn markdown_to_html_fragment(input: &str) -> String {
    // InspiredGitHub
    // Solarized (dark)
    // Solarized (light)
    // base16-eighties.dark
    // base16-mocha.dark
    // base16-ocean.dark
    // base16-ocean.light
    let adapter = SyntectAdapterBuilder::new()
        .theme("base16-ocean.light")
        .build();

    let mut options = comrak::Options::default();
    options.extension.strikethrough = true;
    options.extension.table = true;
    options.extension.tagfilter = true;
    options.extension.tasklist = true;
    options.extension.autolink = true;
    options.extension.footnotes = true;
    options.render.unsafe_ = true; // allow raw HTML
    options.extension.header_ids = Some("user-content-".to_string()); // mimics GitHub's prefix
    options.render.github_pre_lang = true;

    let mut plugins = Plugins::default();

    plugins.render.codefence_syntax_highlighter = Some(&adapter);

    let inner_html = markdown_to_html_with_plugins(input, &options, &plugins);

    format!(r#"<div class="markdown-body">{inner_html}</div>"#)
}

/// Wraps an HTML fragment in a complete HTML5 document shell.
pub fn wrap_in_html_doc(content: &str, title: &str, styles: &[String]) -> String {
    let style_tags: String = styles
        .iter()
        .map(|s| format!(r#"<style>{}</style>"#, s))
        .collect();
    format!(
        r#"<!DOCTYPE html>
            <html lang="en">
            <head>
                <meta charset="UTF-8">
                <title>{title}</title>
                {style_tags}
                <style>
                    html, body {{
                      height: 100%;
                      margin: 0;
                      padding: 0;
                      box-sizing: border-box; /* Optional, but good practice */
                    }}
                </style>
            </head>
            <body>
                {content}
            </body>
            </html>"#
    )
}
