use kuchiki::traits::*;
use kuchiki::parse_html;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use dirs::home_dir;
use std::process::Command;

fn main() -> io::Result<()> {
    // Prompt the user to enter the path to the HTML file
    println!("Please enter the path to your bookmarks HTML file:");

    // Read the user input
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    // Trim whitespace and expand `~` to the home directory
    let input = input.trim();
    let expanded_path = shellexpand::tilde(input).into_owned();

    // Print the file path to verify
    println!("Trying to read file at path: {}", expanded_path);

    // Check if the file exists
    let path = Path::new(&expanded_path);
    if !path.exists() {
        eprintln!("File not found: {}", expanded_path);
        return Err(io::Error::new(io::ErrorKind::NotFound, "File not found"));
    }

    // Read the HTML content
    let html_content = match std::fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return Err(e);
        }
    };

    // Parse the HTML content
    let document = parse_html().one(html_content);

    // Traverse the HTML tree and extract bookmarks
    let mut markdown_content = String::new();
    let bookmark_count = parse_bookmarks(&document, &mut markdown_content, true);

    // Write to markdown file in Downloads directory
    let downloads_dir = home_dir().unwrap().join("Downloads");
    let markdown_file_path = downloads_dir.join("bookmarks.md");
    let mut file = File::create(&markdown_file_path)?;
    file.write_all(markdown_content.as_bytes())?;

    // Try to open with Obsidian, fallback to default text editor
    if Command::new("obsidian").arg(markdown_file_path.to_str().unwrap()).output().is_err() {
        if cfg!(target_os = "macos") {
            Command::new("open").arg(markdown_file_path.to_str().unwrap()).output()?;
        } else if cfg!(target_os = "linux") {
            Command::new("xdg-open").arg(markdown_file_path.to_str().unwrap()).output()?;
        } else if cfg!(target_os = "windows") {
            Command::new("cmd").args(&["/C", markdown_file_path.to_str().unwrap()]).output()?;
        }
    }

    println!("Processed {} bookmarks ✅", bookmark_count);
    Ok(())
}

fn parse_bookmarks(node: &kuchiki::NodeRef, markdown_content: &mut String, is_first: bool) -> usize {
    let mut count = 0;
    for child in node.children() {
        if let Some(element) = child.as_element() {
            let name = element.name.local.to_string();
            if name == "h3" {
                let title = child.text_contents().trim().to_string();
                if !is_first {
                    markdown_content.push_str("\n");
                }
                markdown_content.push_str(&format!("#### {}\n\n| name | url |\n| --- | --- |\n", title));
            } else if name == "a" {
                let url = element.attributes.borrow().get("href").unwrap().to_string();
                let title = child.text_contents().trim().to_string();
                markdown_content.push_str(&format!("| {} | {} |\n", title, url));
                count += 1;
            }
        }
        count += parse_bookmarks(&child, markdown_content, false);
    }
    count
}
