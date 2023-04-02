use std::{fs, io};

pub fn md5_file(path: &str) -> Result<String, io::Error> {
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
