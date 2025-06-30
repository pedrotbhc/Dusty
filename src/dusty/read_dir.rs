use crate::dusty::file_size;
use rayon::prelude::*;
use std::os::unix::fs::MetadataExt;
use walkdir::WalkDir;

#[derive(Clone)]
pub struct FileInfo {
    pub path: String,
    pub byte_size: u64,
    pub human_size: String,
    pub owner_uid: u32,
    pub owner_gid: u32,
    pub inode: u64,
    pub permissions: u32,
    pub nlinks: u64,
    pub dir_info: Option<DirInfo>,
}

#[derive(Clone)]
#[allow(dead_code)]
pub struct DirInfo {
    pub folders_num: u64,
    pub files_num: u64,
}

pub fn read_dir(path: &str) -> Vec<FileInfo> {
    let mut entries: Vec<FileInfo> = WalkDir::new(path)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .par_bridge()
        .filter_map(Result::ok)
        .filter_map(|entry| {
            let metadata = entry.metadata().ok()?;
            let file_info = if metadata.file_type().is_file() {
                FileInfo {
                    path: entry.file_name().to_string_lossy().to_string(),
                    byte_size: metadata.len(),
                    human_size: file_size::convert_bytes(metadata.len()),
                    owner_uid: metadata.uid(),
                    owner_gid: metadata.gid(),
                    inode: metadata.ino(),
                    permissions: metadata.mode(),
                    nlinks: metadata.nlink(),
                    dir_info: None,
                }
            } else {
                FileInfo {
                    path: entry.file_name().to_string_lossy().to_string(),
                    byte_size: metadata.len(),
                    human_size: String::new(),
                    owner_uid: metadata.uid(),
                    owner_gid: 0,
                    inode: metadata.ino(),
                    permissions: metadata.mode(),
                    nlinks: metadata.nlink(),
                    dir_info: None,
                }
            };
            Some(file_info)
        })
        .collect();
    entries.sort_by_key(|e| e.path.to_ascii_lowercase());
    entries
}

pub fn dir_size_byte(path: &std::path::Path) -> u64 {
    let mut bytes_total: u64 = 0;
    for entry in WalkDir::new(path).into_iter().filter_map(Result::ok) {
        if entry.file_type().is_file() {
            if let Ok(meta) = entry.metadata() {
                bytes_total += meta.len();
            }
        }
    }
    bytes_total
}
