use crate::cli::MetadataArgs;
use anyhow::Result;
use colored::*;
use std::fs;
use std::path::Path;
// use kamadak_exif::{Reader, In, Tag};
use pdf_extract::extract_text;
use serde_json::{json, Value};

pub fn run(args: MetadataArgs) -> Result<()> {
    println!("{} Extracting metadata from: {}", "📄".cyan(), args.file.yellow().bold());
    
    let path = Path::new(&args.file);
    
    if !path.exists() {
        println!("{} File not found: {}", "❌".red(), args.file);
        return Ok(());
    }
    
    let metadata = fs::metadata(path)?;
    let file_size = metadata.len();
    let extension = path.extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("unknown")
        .to_lowercase();
    
    println!("\n{} Basic File Information", "📋".cyan());
    println!("{}", "=".repeat(40));
    println!("📁 File: {}", path.file_name().unwrap().to_string_lossy().yellow());
    println!("📏 Size: {} bytes ({:.2} KB)", file_size, file_size as f64 / 1024.0);
    println!("🏷️  Type: {}", extension.cyan());
    
    if let Ok(modified) = metadata.modified() {
        if let Ok(duration) = modified.duration_since(std::time::UNIX_EPOCH) {
            let datetime = chrono::DateTime::from_timestamp(duration.as_secs() as i64, 0)
                .unwrap_or_default();
            println!("📅 Modified: {}", datetime.format("%Y-%m-%d %H:%M:%S UTC"));
        }
    }
    
    match extension.as_str() {
        "jpg" | "jpeg" | "tiff" | "tif" => {
            println!("\n{} JPEG/TIFF image detected", "📸".yellow());
            println!("{} EXIF extraction requires kamadak-exif crate", "⚠️".yellow());
        }
        "pdf" => {
            extract_pdf_metadata(path)?;
        }
        "png" | "gif" | "bmp" => {
            println!("\n{} Image file detected but no EXIF support for {}", "📸".yellow(), extension.to_uppercase());
        }
        "mp4" | "avi" | "mov" | "mkv" => {
            println!("\n{} Video file detected", "🎥".yellow());
            // Video metadata extraction could be added here
        }
        "mp3" | "wav" | "flac" | "ogg" => {
            println!("\n{} Audio file detected", "🎵".yellow());
            // Audio metadata extraction could be added here
        }
        "doc" | "docx" | "xls" | "xlsx" | "ppt" | "pptx" => {
            println!("\n{} Office document detected", "📄".yellow());
            // Office document metadata could be added here
        }
        _ => {
            println!("\n{} No specific metadata extractor for file type: {}", "⚠️".yellow(), extension.to_uppercase());
        }
    }
    
    if let Some(format) = &args.format {
        if format == "json" {
            output_json_format(path, file_size, &extension)?;
        }
    }
    
    Ok(())
}

// EXIF extraction function - disabled until kamadak-exif is added
/*
fn extract_image_metadata(path: &Path) -> Result<()> {
    println!("\n{} EXIF Data", "📸".cyan());
    println!("{}", "=".repeat(40));
    println!("{} EXIF extraction requires additional dependencies", "⚠️".yellow());
    Ok(())
}
*/

fn extract_pdf_metadata(path: &Path) -> Result<()> {
    println!("\n{} PDF Metadata", "📄".cyan());
    println!("{}", "=".repeat(40));
    
    match extract_text(path) {
        Ok(text) => {
            let word_count = text.split_whitespace().count();
            let char_count = text.chars().count();
            let page_estimate = std::cmp::max(1, char_count / 2000);
            
            println!("📊 Text Statistics:");
            println!("   Words: {}", word_count.to_string().yellow());
            println!("   Characters: {}", char_count.to_string().yellow());
            println!("   Estimated Pages: {}", page_estimate.to_string().yellow());
            
            if text.len() > 200 {
                println!("📝 Text Preview:");
                println!("   {}", text.chars().take(200).collect::<String>().trim());
                println!("   {}...", "".dimmed());
            }
        }
        Err(e) => {
            println!("{} Could not extract PDF text: {}", "⚠️".yellow(), e);
        }
    }
    
    Ok(())
}

fn output_json_format(path: &Path, file_size: u64, extension: &str) -> Result<()> {
    let metadata = fs::metadata(path)?;
    
    let mut json_output = json!({
        "file_info": {
            "name": path.file_name().unwrap().to_string_lossy(),
            "size_bytes": file_size,
            "size_kb": file_size as f64 / 1024.0,
            "extension": extension,
            "type": get_file_type(extension)
        }
    });
    
    if let Ok(modified) = metadata.modified() {
        if let Ok(duration) = modified.duration_since(std::time::UNIX_EPOCH) {
            json_output["file_info"]["modified_timestamp"] = json!(duration.as_secs());
        }
    }
    
    println!("\n{} JSON Output:", "💾".cyan());
    println!("{}", serde_json::to_string_pretty(&json_output)?);
    
    Ok(())
}

fn get_file_type(extension: &str) -> &'static str {
    match extension {
        "jpg" | "jpeg" | "png" | "gif" | "bmp" | "tiff" | "tif" => "image",
        "mp4" | "avi" | "mov" | "mkv" | "wmv" | "flv" => "video",
        "mp3" | "wav" | "flac" | "ogg" | "aac" => "audio",
        "pdf" => "document",
        "doc" | "docx" | "txt" | "rtf" => "document", 
        "xls" | "xlsx" | "csv" => "spreadsheet",
        "ppt" | "pptx" => "presentation",
        "zip" | "rar" | "7z" | "tar" | "gz" => "archive",
        _ => "unknown"
    }
}