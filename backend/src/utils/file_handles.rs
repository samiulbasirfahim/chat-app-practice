use actix_multipart::form::tempfile::TempFile;
use std::fs;
use std::path::Path;
use uuid::Uuid;

use crate::get_config;

pub fn upload_avatar(file: &TempFile, subdir: &str) -> Result<String, Box<dyn std::error::Error>> {
    let path = get_config().upload_dir.join(subdir);
    let original_name = file.file_name.as_deref().unwrap_or("unknown");
    let ext = Path::new(original_name)
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("-");
    let file_path = path.join(format!("{}.{}", Uuid::new_v4(), ext));
    fs::copy(file.file.path(), &file_path)?;

    println!("File uploaded to: {:?}", file_path);
    Ok("HELLO".to_string())
}
