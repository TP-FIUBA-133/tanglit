use std::{fmt, fs, path::Path};

use headless_chrome::{Browser, types::PrintToPdfOptions};

pub enum GeneratePdfError {
    IOError(String),
    ChromeError(String),
}

impl fmt::Display for GeneratePdfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GeneratePdfError::IOError(msg) => write!(f, "Invalid input: {}", msg),
            GeneratePdfError::ChromeError(msg) => write!(f, "Chrome error: {}", msg),
        }
    }
}

impl fmt::Debug for GeneratePdfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GeneratePdfError::IOError(msg) => write!(f, "Invalid input: {}", msg),
            GeneratePdfError::ChromeError(msg) => write!(f, "Chrome error: {}", msg),
        }
    }
}

impl From<std::io::Error> for GeneratePdfError {
    fn from(error: std::io::Error) -> Self {
        GeneratePdfError::IOError(format!("IO Error: {}", error))
    }
}

pub fn generate_pdf(html: &str, output_file_path: &str) -> Result<(), GeneratePdfError> {
    let temp_file = Path::new("temp.html");
    fs::write(temp_file, html)?;
    let abs_path = fs::canonicalize(temp_file)?;
    let file_url = format!("file://{}", abs_path.to_string_lossy());

    // Launch headless Chromium
    let browser = Browser::default()
        .map_err(|e| GeneratePdfError::ChromeError(format!("Failed to launch browser: {}", e)))?;

    // Open a new tab and navigate to the file
    let tab = browser
        .new_tab()
        .map_err(|e| GeneratePdfError::ChromeError(format!("Failed to open new tab: {}", e)))?;
    tab.navigate_to(&file_url)
        .map_err(|e| GeneratePdfError::ChromeError(format!("Failed to navigate: {}", e)))?;
    tab.wait_until_navigated().map_err(|e| {
        GeneratePdfError::ChromeError(format!("Failed to wait for navigation: {}", e))
    })?;

    // Print to PDF and save it
    let pdf_data = tab
        .print_to_pdf(Some(PrintToPdfOptions {
            print_background: Some(true),
            ..Default::default()
        }))
        .map_err(|e| GeneratePdfError::ChromeError(format!("Failed to print to PDF: {}", e)))?;

    std::fs::write(output_file_path, pdf_data)?;
    std::fs::remove_file(temp_file)?; // Clean up temp file
    Ok(())
}
