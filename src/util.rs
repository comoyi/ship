use std::{fs, io};

pub fn md5_file(path: &str) -> Result<String, io::Error> {
    let f_r = fs::read(path);
    let f;
    match f_r {
        Ok(file) => f = file,
        Err(e) => {
            return Err(e);
        }
    }
    let s = md5::compute(f);
    Ok(format!("{:x}", s))
}

pub fn md5sum(s: &str) -> String {
    let s = md5::compute(s);
    format!("{:x}", s)
}
