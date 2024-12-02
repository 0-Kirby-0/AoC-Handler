use std::{
    fs::{self, File},
    io,
    path::PathBuf,
};

pub fn is_cached(sub_path: &PathBuf) -> bool {
    file_dir(sub_path).exists()
}

pub fn cache<'a>(sub_path: &PathBuf, data: impl Into<&'a [u8]>) -> io::Result<()> {
    std::fs::write(file_dir(sub_path), data.into())
}

pub fn get_cached(sub_path: &PathBuf) -> File {
    std::fs::File::open(file_dir(sub_path)).expect("Unable to retrieve cached file.")
}

fn file_dir(sub_path: &PathBuf) -> PathBuf {
    let path = cache_dir().join(sub_path);
    fs::create_dir_all(path.parent().unwrap()).expect("Unable to create requested cache dirs.");
    path
}

fn cache_dir() -> PathBuf {
    dirs::cache_dir()
        .expect("Could not find cache location.")
        .join("aoc_input")
}
