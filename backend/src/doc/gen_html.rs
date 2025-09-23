use comrak::plugins::syntect::SyntectAdapterBuilder;
use comrak::{Plugins, markdown_to_html_with_plugins};

// Taken from https://github.com/sindresorhus/github-markdown-css/blob/bedb4b771f5fa1ae117df597c79993fd1eb4dff0/github-markdown-light.css
const GITHUB_MARKDOWN_LIGHT_CSS: &str = include_str!("../../resources/github-markdown-light.css");

// TODO: Make all options configurable
pub fn markdown_to_html(input: &str) -> String {
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
    options.extension.header_ids = Some("user-content-".to_string()); // mimics GitHub's prefix
    options.render.github_pre_lang = true;

    let mut plugins = Plugins::default();

    plugins.render.codefence_syntax_highlighter = Some(&adapter);

    let inner_html = markdown_to_html_with_plugins(input, &options, &plugins);

    let content_html = format!(r#"<div class="markdown-body">{inner_html}</div>"#);

    wrap_in_html_doc(
        &content_html,
        "Document", // TODO get title from arg or extract from markdown
        &[GITHUB_MARKDOWN_LIGHT_CSS.to_string()],
    )
}

/// Wraps an HTML fragment in a complete HTML5 document shell.
fn wrap_in_html_doc(content: &str, title: &str, styles: &[String]) -> String {
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
