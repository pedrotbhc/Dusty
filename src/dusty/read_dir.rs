use std::fs;
use std::os::unix::fs::MetadataExt;
use walkdir::WalkDir;

pub struct FileInfo {
    pub path: String,
    pub size: u64,
    pub owner_uid: u32,
    pub is_dir: bool,
}

pub fn read_dir(path: &str) -> Vec<FileInfo> {
    let mut files = Vec::new();

    for entry in WalkDir::new(path).min_depth(1).max_depth(1) {
        if let Ok(entry) = entry {
            let metadata = match fs::metadata(entry.path()) {
                Ok(m) => m,
                Err(_) =>  continue,
            };

            files.push(FileInfo {
                path: entry.file_name().to_string_lossy().to_string(),
                size: metadata.len(),
                owner_uid: metadata.uid(),
                is_dir: metadata.is_dir(),
            });
        }
    }
    files
} 
