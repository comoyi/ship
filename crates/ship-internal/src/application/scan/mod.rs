use crate::types::common::{ClientFileInfo, FileInfo, FileType, ScanStatus};
use chrono::Local;
use log::{debug, warn};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[derive(Debug)]
pub enum Error {
    ScanError,
    PathNotExitError,
    CalcHashError,

    Cancel,
}

pub fn scan(base_path: &str, is_cancel: Arc<AtomicBool>) -> Result<ClientFileInfo, Error> {
    let p = Path::new(base_path);
    if !p.exists() {
        return Err(Error::PathNotExitError);
    }
    let mut files: Vec<FileInfo> = vec![];

    debug!("{}", "scan start");

    let d = walkdir::WalkDir::new(base_path);

    let iter = d.into_iter();
    for entry_r in iter {
        if is_cancel.load(Ordering::Relaxed) {
            return Err(Error::Cancel);
        }
        match entry_r {
            Ok(entry) => {
                let absolute_path = entry.path().to_str().unwrap();
                if absolute_path == base_path {
                    debug!("ignore base_path");
                    continue;
                }
                let relative_path = match Path::new(absolute_path).strip_prefix(base_path) {
                    Ok(p) => match p.to_str() {
                        None => {
                            return Err(Error::ScanError);
                        }
                        Some(ps) => ps,
                    },
                    Err(_) => {
                        return Err(Error::ScanError);
                    }
                };
                let file_type;
                let mut size = 0;
                let mut hash_sum = "".to_string();
                if entry.path().is_symlink() {
                    file_type = FileType::Symlink;
                    size = entry.metadata().unwrap().len();
                } else if entry.path().is_dir() {
                    file_type = FileType::Dir;
                } else if entry.path().is_file() {
                    file_type = FileType::File;
                    size = entry.metadata().unwrap().len();
                    let hash_sum_r = util::hash::md5::md5_file(absolute_path);
                    match hash_sum_r {
                        Ok(h) => {
                            hash_sum = h;
                        }
                        Err(e) => {
                            warn!("calc hash failed, err: {}, rel_path: {}", e, relative_path);
                            return Err(Error::CalcHashError);
                        }
                    }
                } else {
                    warn!("ignored file type, relative_path: {}", relative_path);
                    continue;
                }

                let file = FileInfo::new(&relative_path, file_type, size, &hash_sum);
                debug!(
                    "abs_path: {}, rel_path: {}, file: {:?}",
                    absolute_path, relative_path, file
                );
                files.push(file);
            }
            Err(_) => {
                return Err(Error::ScanError);
            }
        }
    }

    let cfi: ClientFileInfo =
        ClientFileInfo::new(ScanStatus::Completed, Local::now().timestamp(), files);

    debug!("ClientFileInfo: {:?}", cfi);
    debug!("scan completed");

    Ok(cfi)
}
