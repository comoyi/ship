use crate::data::{ClientFileInfo, FileInfo, FileType, ScanStatus};
use crate::error::Error;
use crate::util;
use chrono::Local;
use log::{debug, warn};
use std::path::Path;

pub fn scan(base_path: &str) -> Result<ClientFileInfo, Error> {
    let mut files: Vec<FileInfo> = vec![];

    debug!("{}", "scan start");

    let d = walkdir::WalkDir::new(base_path);

    let iter = d.into_iter();
    for entry_res in iter {
        match entry_res {
            Ok(entry) => {
                let absolute_path = entry.path().to_str().unwrap();
                if absolute_path == base_path {
                    debug!("ignore base_path")
                } else {
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
                        let hash_sum_r = util::md5_file(absolute_path);
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

                    debug!("abs_path: {}, rel_path: {}", absolute_path, relative_path);
                    let mut file = FileInfo::new();
                    file.relative_path = relative_path.to_string();
                    file.file_type = file_type;
                    file.size = size;
                    file.hash = hash_sum;
                    files.push(file);
                }
            }
            Err(_) => {
                return Err(Error::ScanError);
            }
        }
    }

    let mut cfi: ClientFileInfo = Default::default();
    cfi.files = files;
    cfi.scan_status = ScanStatus::Completed;
    cfi.last_scan_finish_time = Local::now().timestamp();

    let mut j = String::from("");
    let j_r = serde_json::to_string(&cfi);
    match j_r {
        Ok(js) => {
            j = js;
        }
        Err(_) => {
            warn!("json serialize failed");
        }
    }

    debug!("json: {}", j);
    debug!("{}", "scan completed");

    Ok(cfi)
}
