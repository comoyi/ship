use std::path::Path;
use std::{fs, io};

pub fn md5_file<P: AsRef<Path>>(path: P) -> Result<String, io::Error> {
    let file_data_r = fs::read(path);
    let file_data = match file_data_r {
        Ok(d) => d,
        Err(e) => {
            return Err(e);
        }
    };
    let s = md5::compute(file_data);
    Ok(format!("{:x}", s))
}

pub fn md5_string(str: &str) -> String {
    let s = md5::compute(str);
    format!("{:x}", s)
}
