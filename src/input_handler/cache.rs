use core::panic;
use std::path::{Path, PathBuf};

pub fn is_cached(sub_path: &Path) -> bool {
    file_path(sub_path).exists()
}

pub fn cache(sub_path: &Path, data: impl AsRef<[u8]>) -> std::io::Result<()> {
    std::fs::write(file_path(sub_path), data)
}

pub fn get_cached(sub_path: &Path) -> std::io::Result<std::fs::File> {
    std::fs::File::open(file_path(sub_path))
}

pub fn clear_cached(sub_path: &Path) -> std::io::Result<()> {
    std::fs::remove_file(file_path(sub_path))
}

fn file_path(sub_path: &Path) -> PathBuf {
    let path = dirs::cache_dir()
        .expect("dirs did not provide a cache location.")
        .join("aoc_handler") //Parent folder for this cache
        .join(sub_path);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap_or_else(|_| {
        panic!(
            "Unable to create requested cache directory '{}'",
            path.display()
        )
    });

    path
}

pub mod text {
    use anyhow::Context;
    use std::io::Read;
    use std::path::Path;

    pub fn cache(sub_path: &Path, text: &str) -> std::io::Result<()> {
        if text.ends_with('\n') {
            super::cache(sub_path, text)
        } else {
            let mut text = text.to_owned();
            text.push('\n');
            super::cache(sub_path, text)
        }
    }

    pub fn get_cached(sub_path: &Path) -> anyhow::Result<String> {
        let mut text = String::default();
        super::get_cached(sub_path)?
            .read_to_string(&mut text)
            .with_context(|| {
                format!(
                    "Unable to read cached file at '{}' to string.",
                    sub_path.display()
                )
            })?;
        Ok(text.trim().to_owned())
    }
}
