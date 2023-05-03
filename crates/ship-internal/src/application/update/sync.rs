use crate::cache;
use crate::types::common::{DataNode, FileInfo, FileType};
use log::{debug, warn};
use rand::{thread_rng, Rng};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::{fs, io};
use util::hash;

#[derive(Clone, Debug)]
pub struct SyncTask {
    pub app_id: u64,
    pub sync_type: SyncTaskType,
    pub file_info: FileInfo,
    pub base_path: String,
    pub data_nodes: Vec<DataNode>,
}

impl SyncTask {
    pub fn new(
        app_id: u64,
        sync_type: SyncTaskType,
        file_info: FileInfo,
        base_path: String,
        data_nodes: Vec<DataNode>,
    ) -> Self {
        Self {
            app_id,
            sync_type,
            file_info,
            base_path,
            data_nodes,
        }
    }
}

#[derive(Clone, Debug)]
pub enum SyncTaskType {
    Create,
    Update,
    Delete,
}

#[derive(Debug)]
pub enum SyncError {
    PathInvalid,
    DownloadFailed,
    CreateFileFailed,
    ReadDownloadContentFailed,
    WriteDownloadContentFailed,
    AddToCacheFailed,
    SyncFromCacheError,
    CreateDirFailed,
    CreateSymlinkFailed,

    HashSumSyncedFileError,
    SyncedFileHashSumNotMatch,
    DeleteFailed,
    UnknownFileType,
    CheckExistsFailed,
    Other,
}

pub fn handle_task(task: SyncTask) -> Result<(), SyncError> {
    debug!("SyncTask: {:?}", task);

    match task.sync_type {
        SyncTaskType::Create | SyncTaskType::Update => {
            debug!("will sync, file_info: {:?}", task.file_info);

            let full_file_path = Path::new(&task.base_path).join(&task.file_info.relative_path);
            if !full_file_path.starts_with(&task.base_path) {
                return Err(SyncError::PathInvalid);
            }
            let r = delete_file(&full_file_path);
            if let Err(e) = r {
                return Err(e);
            }

            match task.file_info.file_type {
                FileType::Unknown => {
                    return Err(SyncError::UnknownFileType);
                }
                FileType::File => {
                    let cache_file_o = cache::get_cache_file(&task.file_info.hash);
                    match cache_file_o {
                        None => {
                            let index = thread_rng().gen_range(0..task.data_nodes.len());
                            let url = format!(
                                "{}/api/v1/download?file={}",
                                task.data_nodes
                                    .get(index)
                                    .unwrap()
                                    .address
                                    .to_address_string(),
                                task.file_info.relative_path
                            );
                            let mut resp = reqwest::blocking::get(url)
                                .map_err(|e| SyncError::DownloadFailed)?;
                            let f = fs::File::create(&full_file_path)
                                .map_err(|e| SyncError::CreateFileFailed)?;
                            let mut writer = io::BufWriter::new(f);
                            let mut buf = [0; 1024 * 1024];
                            loop {
                                let n = resp
                                    .read(&mut buf)
                                    .map_err(|e| SyncError::ReadDownloadContentFailed)?;
                                if n == 0 {
                                    break;
                                }
                                writer
                                    .write(&buf[..n])
                                    .map_err(|e| SyncError::WriteDownloadContentFailed)?;
                            }
                            writer.flush().map_err(|e| SyncError::CreateFileFailed)?;
                            check_hash(&task, &full_file_path)?;
                            cache::add_to_cache(&full_file_path, task.app_id)
                                .map_err(|e| SyncError::AddToCacheFailed)?;
                        }
                        Some(cache_file) => {
                            let cache_dir = cache::get_update_cache_dir_path()
                                .map_err(|e| SyncError::SyncFromCacheError)?;
                            let cache_file_path =
                                Path::new(&cache_dir).join(cache_file.relative_path);
                            fs::copy(cache_file_path, &full_file_path)
                                .map_err(|e| SyncError::SyncFromCacheError)?;
                            check_hash(&task, &full_file_path)?;
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
                        "{}/api/v1/download?file={}",
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
                            util::fs::symlink::symlink_dir(&content, &full_file_path);
                    } else {
                        create_symlink_r =
                            util::fs::symlink::symlink_file(&content, &full_file_path);
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

fn check_hash(task: &SyncTask, full_file_path: &PathBuf) -> Result<bool, SyncError> {
    // check hash
    let hash_sum =
        hash::md5::md5_file(&full_file_path).map_err(|e| SyncError::HashSumSyncedFileError)?;
    if task.file_info.hash != hash_sum {
        warn!(
            "synced file hash != file info hash, hash: {}, file_info: {:?}",
            hash_sum, task.file_info
        );
        return Err(SyncError::SyncedFileHashSumNotMatch);
    } else {
        debug!(
            "synced file hash == file info hash, hash: {}, file_info: {:?}",
            hash_sum, task.file_info
        );
    }
    Ok(true)
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
