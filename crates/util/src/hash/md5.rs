use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{fs, io};

pub fn md5_file<P: AsRef<Path>>(path: P) -> Result<String, io::Error> {
    let file_data = fs::read(path)?;
    let s = md5::compute(file_data);
    Ok(format!("{:x}", s))
}

pub fn md5_file_cancelable<P: AsRef<Path>>(
    path: P,
    is_cancel: Arc<AtomicBool>,
) -> Result<String, io::Error> {
    let f = File::open(path)?;
    let default_capacity = 64 * 1024;
    let capacity = f
        .metadata()
        .and_then(|x| {
            if x.len() < default_capacity {
                return Ok(x.len());
            }
            return Ok(default_capacity);
        })
        .unwrap_or(default_capacity);
    let mut reader = io::BufReader::with_capacity(capacity as usize, f);
    let mut buf = [0; 256 * 1024];
    let mut file_data: Vec<u8> = vec![];
    loop {
        if is_cancel.load(Ordering::Relaxed) {
            return Err(io::ErrorKind::Other.into());
        }

        let n = reader.read(&mut buf)?;
        if n == 0 {
            break;
        }
        file_data.append(&mut buf[..n].to_vec());
    }
    let s = md5::compute(file_data);
    Ok(format!("{:x}", s))
}

pub fn md5_string(str: &str) -> String {
    let s = md5::compute(str);
    format!("{:x}", s)
}

#[cfg(test)]
mod test {
    use crate::hash::md5::{md5_file, md5_file_cancelable};
    use std::sync::atomic::AtomicBool;
    use std::sync::Arc;

    #[test]
    fn test_eq() {
        let p = "README.md";
        let a = md5_file(p).unwrap();
        let is_cancel = Arc::new(AtomicBool::new(false));
        let b = md5_file_cancelable(p, is_cancel).unwrap();
        assert_eq!(a, b, "md5_file->a: {}, md5_file_cancelable->b: {}", a, b);
    }
}
