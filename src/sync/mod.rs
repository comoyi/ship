use crate::data::common::{FileType, SyncTask, SyncTaskType};
use crate::error::SyncError;
use crate::utils::hash;
use crate::{cache, utils};
use log::{debug, warn};
use rand::{thread_rng, Rng};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::time::Duration;
use std::{fs, io, thread};

pub fn handle_task(task: &SyncTask) -> Result<(), SyncError> {
    debug!("handel task: {:?}", task);
    thread::sleep(Duration::from_millis(1000));
    match task.sync_type {
        SyncTaskType::Create | SyncTaskType::Update => {
            debug!("will sync, file_info: {:?}", task.file_info);

            let full_file_path = Path::new(&task.base_path).join(&task.file_info.relative_path);
            let r = delete_file(&full_file_path);
            if let Err(e) = r {
                return Err(e);
            }

            match task.file_info.file_type {
                FileType::Unknown => {
                    return Err(SyncError::UnknownFileType);
                }
                FileType::File => {
                    let url = format!(
                        "{}/api/v1/download/{}",
                        task.data_nodes.get(0).unwrap().address.to_address_string(),
                        task.file_info.relative_path
                    );
                    let resp_r = reqwest::blocking::get(url);
                    match resp_r {
                        Ok(mut resp) => {
                            let f_r = fs::File::create(&full_file_path);
                            match f_r {
                                Ok(f) => {
                                    let mut writer = io::BufWriter::new(f);
                                    let mut buf = [0; 1024 * 1024];
                                    loop {
                                        let r = resp.read(&mut buf);
                                        match r {
                                            Ok(n) => {
                                                if n == 0 {
                                                    break;
                                                }
                                                let r = writer.write(&buf[..n]);
                                                if let Err(e) = r {
                                                    return Err(
                                                        SyncError::ReadDownloadContentFailed,
                                                    );
                                                }
                                            }
                                            Err(_) => {
                                                return Err(SyncError::ReadDownloadContentFailed);
                                            }
                                        }
                                    }
                                    let r = writer.flush();
                                    if let Err(e) = r {
                                        return Err(SyncError::CreateFileFailed);
                                    }

                                    // check hash
                                    let hash_r = hash::md5::md5_file(&full_file_path);
                                    match hash_r {
                                        Ok(hash_sum) => {
                                            if task.file_info.hash != hash_sum {
                                                warn!("synced file hash != file info hash, hash: {}, file_info: {:?}",hash_sum,task.file_info);
                                                return Err(SyncError::SyncedFileHashError);
                                            } else {
                                                debug!("synced file hash == file info hash, hash: {}, file_info: {:?}",hash_sum,task.file_info);
                                            }

                                            // cache
                                            cache::add_to_cache(&full_file_path);
                                        }
                                        Err(_) => {
                                            return Err(SyncError::CheckSyncedFileError);
                                        }
                                    }
                                }
                                Err(_) => {
                                    return Err(SyncError::CreateFileFailed);
                                }
                            }
                        }
                        Err(_) => {
                            return Err(SyncError::DownloadFailed);
                        }
                    }
                }
                FileType::Dir => {
                    let create_dir_r = fs::create_dir_all(&full_file_path);
                    if let Err(e) = create_dir_r {
                        warn!(
                            "create dir failed, full_file_path: {:?}, err: {}",
                            full_file_path, e
                        );
                        return Err(SyncError::CreateDirFailed);
                    }
                }
                FileType::Symlink => {
                    let mut content = "".to_string();
                    let index = thread_rng().gen_range(0..task.data_nodes.len());
                    let url = format!(
                        "{}/api/v1/download/{}",
                        task.data_nodes
                            .get(index)
                            .unwrap()
                            .address
                            .to_address_string(),
                        task.file_info.relative_path
                    );
                    let resp_r = reqwest::blocking::get(url);
                    match resp_r {
                        Ok(mut resp) => {
                            let read_r = resp.read_to_string(&mut content);
                            if let Err(e) = read_r {
                                return Err(SyncError::ReadDownloadContentFailed);
                            }
                        }
                        Err(_) => {
                            return Err(SyncError::DownloadFailed);
                        }
                    }
                    let original_path = Path::new(&content);
                    let create_symlink_r;
                    if original_path.is_dir() {
                        create_symlink_r =
                            utils::fs::symlink::symlink_dir(&content, &full_file_path);
                    } else {
                        create_symlink_r =
                            utils::fs::symlink::symlink_file(&content, &full_file_path);
                    }
                    if let Err(e) = create_symlink_r {
                        warn!(
                            "create symlink failed, full_file_path: {:?}, err: {}",
                            full_file_path, e
                        );
                        return Err(SyncError::CreateSymlinkFailed);
                    }
                }
            }
        }
        SyncTaskType::Delete => {
            debug!("will delete, file_info: {:?}", task.file_info);
            let full_file_path = Path::new(&task.base_path).join(&task.file_info.relative_path);
            debug!(
                "will delete, file_info: {:?}, full_file_path: {:?}",
                task.file_info, full_file_path
            );
            let r = delete_file(&full_file_path);
            if let Err(e) = r {
                return Err(e);
            }
        }
    }
    Ok(())
}

fn delete_file(full_file_path: &PathBuf) -> Result<(), SyncError> {
    let full_file_path_exists_r = full_file_path.try_exists();
    match full_file_path_exists_r {
        Ok(exists) => {
            let is_symlink = full_file_path.is_symlink();
            // symlink is special!
            if is_symlink {
                warn!("[DEL][Symlink]{:?}", full_file_path);
                let r = fs::remove_file(&full_file_path);
                if let Err(e) = r {
                    warn!("[DEL][Symlink][Failed]{:?}, err: {}", full_file_path, e);
                    return Err(SyncError::DeleteFailed);
                }
            } else {
                if exists {
                    if is_symlink {
                        // symlink is special!
                    } else if full_file_path.is_dir() {
                        warn!("[DEL][Dir]{:?}", full_file_path);
                        let r = fs::remove_dir(&full_file_path);
                        if let Err(e) = r {
                            warn!("[DEL][Dir][Failed]{:?}, err: {}", full_file_path, e);
                            return Err(SyncError::DeleteFailed);
                        }
                    } else if full_file_path.is_file() {
                        warn!("[DEL][File]{:?}", full_file_path);
                        let r = fs::remove_file(&full_file_path);
                        if let Err(e) = r {
                            warn!("[DEL][File][Failed]{:?}, err: {}", full_file_path, e);
                            return Err(SyncError::DeleteFailed);
                        }
                    } else {
                        return Err(SyncError::UnknownFileType);
                    }
                }
            }
        }
        Err(_) => {
            return Err(SyncError::CheckExistsFailed);
        }
    }
    Ok(())
}
