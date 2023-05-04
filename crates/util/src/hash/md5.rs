use std::path::Path;
use std::{fs, io};

pub fn md5_file<P: AsRef<Path>>(path: P) -> Result<String, io::Error> {
    let file_data = fs::read(path)?;
    let s = md5::compute(file_data);
    Ok(format!("{:x}", s))
}

pub fn md5_string(str: &str) -> String {
    let s = md5::compute(str);
    format!("{:x}", s)
}
