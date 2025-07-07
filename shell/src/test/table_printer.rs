use std::fs;
use std::time::SystemTime;

use chrono::{DateTime, Local};
use colored::*;
use tabled::{builder::Builder, settings::{Style, Modify, object::Rows}, Table};
use humantime::format_duration;

pub fn tabel_tester_2() {
    let entries = fs::read_dir(".").unwrap();

    let mut builder = Builder::default();
    builder.push_record(["#", "name", "type", "size", "modified"]);

    for (i, entry) in entries.enumerate() {
        let entry = entry.unwrap();
        let metadata = entry.metadata().unwrap();
        let file_type = if metadata.is_dir() { "dir" } else { "file" };

        let name = entry.file_name().into_string().unwrap();
        // Store the plain name for table calculation
        let name_for_table = name.clone();

        let size = format_size(metadata.len());

        let modified_time = metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH);
        let duration = SystemTime::now().duration_since(modified_time).unwrap();
        let modified_human = format_duration(duration).to_string();

        builder.push_record([
            i.to_string(),
            name_for_table, // Use plain name for proper column width calculation
            file_type.to_string(),
            size,
            modified_human,
        ]);
    }

    let table: Table = builder.build()
        .with(Style::modern())
        .with(Modify::new(Rows::new(1..)).with(tabled::settings::Alignment::left())).to_owned();

    // Get the table as string and then apply colors
    let table_string = table.to_string();
    print_colored_table(&table_string);
}

fn print_colored_table(table_string: &str) {
    for line in table_string.lines() {
        let mut colored_line = String::new();
        let mut in_name_column = false;
        let mut current_word = String::new();
        let mut column_count = 0;
        
        for ch in line.chars() {
            if ch == '│' {
                if !current_word.is_empty() {
                    if column_count == 1 && !current_word.trim().is_empty() {
                        // This is the name column, apply coloring
                        let trimmed = current_word.trim();
                        if trimmed == "name" {
                            colored_line.push_str(&current_word);
                        } else {
                            let colored_name = color_filename(trimmed);
                            // Preserve the spacing
                            let spaces_before = current_word.len() - current_word.trim_start().len();
                            let spaces_after = current_word.len() - current_word.trim_end().len();
                            colored_line.push_str(&" ".repeat(spaces_before));
                            colored_line.push_str(&colored_name);
                            colored_line.push_str(&" ".repeat(spaces_after));
                        }
                    } else {
                        colored_line.push_str(&current_word);
                    }
                    current_word.clear();
                }
                colored_line.push(ch);
                column_count += 1;
            } else {
                current_word.push(ch);
            }
        }
        
        // Handle the last column
        if !current_word.is_empty() {
            colored_line.push_str(&current_word);
        }
        
        println!("{}", colored_line);
    }
}

fn color_filename(name: &str) -> String {
    // Determine file type and apply appropriate color
    if name.starts_with('.') && name != ".." {
        // Hidden files - dim
        name.dimmed().to_string()
    } else if name == "cotsh" {
        // Executable - red
        name.red().to_string()
    } else if name.ends_with(".md") || name.ends_with(".txt") {
        // Documentation - yellow
        name.yellow().to_string()
    } else if name.ends_with(".toml") || name.ends_with(".lock") {
        // Config files - magenta
        name.magenta().to_string()
    } else if is_directory_name(name) {
        // Directories - bright cyan
        name.bright_cyan().to_string()
    } else {
        // Regular files - green
        name.green().to_string()
    }
}

fn is_directory_name(name: &str) -> bool {
    // You might want to enhance this logic
    // For now, we'll use common directory names
    matches!(name, "src" | "target" | "extension" | "shell" | ".git" | ".github" | ".idea")
}

fn format_size(size: u64) -> String {
    match size {
        0..=999 => format!("{} B", size),
        1_000..=999_999 => format!("{:.1} kB", size as f64 / 1_000.0),
        _ => format!("{:.1} MB", size as f64 / 1_000_000.0),
    }
}