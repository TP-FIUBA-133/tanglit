use std::{fs, path::Path};

use headless_chrome::{Browser, types::PrintToPdfOptions};

pub fn generate_pdf(html: &str, output_file_path: &str) {
    let temp_file = Path::new("temp.html");
    fs::write(temp_file, html).unwrap();
    let abs_path = fs::canonicalize(temp_file).unwrap();
    let file_url = format!("file://{}", abs_path.to_string_lossy());

    // Launch headless Chromium
    let browser = Browser::default().unwrap();

    // Open a new tab and navigate to the file
    let tab = browser.new_tab().unwrap();
    tab.navigate_to(&file_url).unwrap();
    tab.wait_until_navigated().unwrap();

    // Print to PDF and save it
    let pdf_data = tab
        .print_to_pdf(Some(PrintToPdfOptions {
            print_background: Some(true),
            ..Default::default()
        }))
        .unwrap();
    std::fs::write(output_file_path, pdf_data).unwrap();
    std::fs::remove_file(temp_file).unwrap(); // Clean up temp file
}
