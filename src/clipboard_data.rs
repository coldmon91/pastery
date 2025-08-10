
use std::fs::File;

/**
 * clip.data file
 * format : date-time, sequence, clipboard_content
 */

pub struct ClipboardData {
    path: String,
    file: File,
}

impl ClipboardData {
    pub fn new(path: String) -> Self {
        let dir = std::path::Path::new(&path).parent();
        if let Some(dir) = dir {
            if !dir.exists() {
                std::fs::create_dir_all(dir).expect(&format!("Failed to create directory {}", dir.display()));
            }
        }
        let file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .expect(&format!("Failed to open file {}", &path));
        ClipboardData {
            path,
            file 
        }
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }
}
pub fn write(text: &str) {
}