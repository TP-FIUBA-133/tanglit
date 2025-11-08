use comrak::plugins::syntect::SyntectAdapterBuilder;
use comrak::{Plugins, markdown_to_html_with_plugins};
use log::warn;
use std::string::ToString;

// Taken from https://github.com/sindresorhus/github-markdown-css/blob/bedb4b771f5fa1ae117df597c79993fd1eb4dff0/github-markdown-light.css
pub const GITHUB_MARKDOWN_LIGHT_CSS: &str =
    include_str!("../../resources/github-markdown-light.css");

pub const DEFAULT_THEME: &str = "pico";

pub const PICO_CSS: &str = include_str!("../../resources/pico.min.css");

pub const WATER_CSS: &str = include_str!("../../resources/water.min.css");

pub const SAKURA_CSS: &str = include_str!("../../resources/sakura.css");

pub const LATEX_CSS: &str = include_str!("../../resources/style.min.css");

pub const PAGE_BREAK_AND_CENTER_CSS: &str =
    include_str!("../../resources/page_break_and_center.css");

pub const AVAILABLE_THEMES: &[&str; 4] = &["pico", "water", "sakura", "latex"];
pub const THEME_CSS: &[(&str, &str); 4] = &[
    ("pico", PICO_CSS),
    ("water", WATER_CSS),
    ("sakura", SAKURA_CSS),
    ("latex", LATEX_CSS),
];

fn get_theme_css(theme: &str) -> Option<&'static str> {
    THEME_CSS
        .iter()
        .find_map(|(k, v)| if *k == theme { Some(*v) } else { None })
}

pub fn markdown_to_html(input: &str, theme: &str) -> String {
    let mut final_theme = theme.to_string();
    if !AVAILABLE_THEMES.contains(&theme) {
        warn!(
            "Theme '{}' is not available. Available themes: {:?}",
            theme, AVAILABLE_THEMES
        );
        warn!("Falling back to default theme {}", DEFAULT_THEME);
        final_theme = DEFAULT_THEME.to_string();
    }
    let fragment = markdown_to_html_fragment(input);
    wrap_in_html_doc(
        &fragment,
        "Document", // TODO get title from arg or extract from markdown
        &[get_theme_css(final_theme.as_str()).unwrap().to_string()],
    )
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
    options.extension.header_ids = Some("user-content-".to_string()); // mimics GitHub's prefix
    options.render.github_pre_lang = true;

    let mut plugins = Plugins::default();

    plugins.render.codefence_syntax_highlighter = Some(&adapter);

    markdown_to_html_with_plugins(input, &options, &plugins)
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
            </head>
            <body>
                {content}
            </body>
            </html>"#
    )
}
