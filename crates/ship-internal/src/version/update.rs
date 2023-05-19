use crate::version::version_manage::VersionManager;
use crate::{request, version};
use log::{debug, info, warn};
use std::io::{Read, Write};
use std::path::Path;
use std::process::Command;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::mpsc::Sender;
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;
use std::{env, fs, io, process, thread};

#[derive(Default)]
pub struct Progress {
    pub value: u64,
    pub total: u64,
}

#[derive(Default)]
pub enum UpdateStatus {
    #[default]
    Wait,
    Processing {
        progress: Progress,
    },
    DownloadFinished,
    Canceled,
    Failed,
    Finished,
}

#[derive(Debug)]
enum Error {
    GetNewVersionInfoFailed,
    DownloadFailed,
    CreateFileFailed,
    ReadDownloadContentFailed,
    WriteDownloadContentFailed,
    GetCurrentFileFailed,
    GetCurrentDirFailed,
    GetFilenameFailed,
    RenameOldFailed,
    RenameNewFailed,
    RemoveLegacyFileFailed,

    SendMessageFailed,
}

pub fn update_new_version(version_manager: Arc<Mutex<VersionManager>>) {
    thread::spawn(move || {
        let mut version_manager_g = version_manager.lock().unwrap();
        if version_manager_g.is_updating {
            debug!("last update is processing");
            return;
        }
        version_manager_g.is_updating = true;
        drop(version_manager_g);

        let (tx, rx) = mpsc::channel::<UpdateStatus>();

        let version_manager_1 = Arc::clone(&version_manager);
        thread::spawn(move || loop {
            thread::sleep(Duration::from_millis(200));
            if let Ok(m) = rx.recv() {
                let mut version_manager_g = version_manager_1.lock().unwrap();
                version_manager_g.update_status = m;
                drop(version_manager_g);
            } else {
                return;
            }
        });

        let r = do_update_new_version(tx.clone());
        if let Err(e) = r {
            match e {
                // ignore this error
                Error::RemoveLegacyFileFailed => {
                    warn!("remove legacy file failed");
                }
                _ => {
                    warn!("update new version failed, err: {:?}", e);
                    let mut version_manager_g = version_manager.lock().unwrap();
                    version_manager_g.is_updating = false;
                    drop(version_manager_g);

                    let _ = tx.send(UpdateStatus::Failed);
                    drop(tx);
                    return;
                }
            }
        }
        info!("update new version finished");
        let _ = tx.send(UpdateStatus::Finished);
        let mut version_manager_g = version_manager.lock().unwrap();
        version_manager_g.is_updating = false;
        version_manager_g.is_completed = true;
        drop(version_manager_g);
    });
}

fn do_update_new_version(tx: Sender<UpdateStatus>) -> Result<(), Error> {
    let new_version_info =
        request::check_update::check_update().map_err(|_| Error::GetNewVersionInfoFailed)?;

    let download_url = new_version_info.new_version.download_url;
    let file_path = env::current_exe().map_err(|_| Error::GetCurrentFileFailed)?;
    debug!("file_path: {:?}", file_path);
    let dir_path = file_path.parent().ok_or(Error::GetCurrentDirFailed)?;
    debug!("dir_path: {:?}", dir_path);
    let tmp_file_path = dir_path.join("update-tmp");
    debug!("tmp_file_path: {:?}", tmp_file_path);
    do_download(&download_url, &tmp_file_path, tx.clone())?;
    tx.send(UpdateStatus::DownloadFinished)
        .map_err(|_| Error::SendMessageFailed)?;

    let file_name = file_path
        .file_name()
        .ok_or(Error::GetFilenameFailed)?
        .to_str()
        .ok_or(Error::GetFilenameFailed)?
        .to_string();
    let legacy_file_name = format!("legacy-{}-{}", version::VERSION_TEXT, file_name);

    let legacy_file_path = dir_path.join(legacy_file_name);
    debug!("legacy_file_path: {:?}", legacy_file_path);

    fs::rename(&file_path, &legacy_file_path).map_err(|_| Error::RenameOldFailed)?;
    debug!("renamed, from: {:?}, to: {:?}", file_path, legacy_file_path);

    fs::rename(&tmp_file_path, &file_path).map_err(|_| Error::RenameNewFailed)?;
    debug!("renamed, from: {:?}, to: {:?}", tmp_file_path, file_path);

    fs::remove_file(&legacy_file_path).map_err(|_| Error::RemoveLegacyFileFailed)?;
    debug!(
        "removed legacy file, legacy_file_path: {:?}",
        legacy_file_path
    );

    Ok(())
}

fn do_download<P: AsRef<Path>>(
    download_url: &str,
    file_path: P,
    tx: Sender<UpdateStatus>,
) -> Result<(), Error> {
    let mut resp = reqwest::blocking::get(download_url).map_err(|_| Error::DownloadFailed)?;
    let f = fs::File::create(file_path).map_err(|_| Error::CreateFileFailed)?;
    let mut writer = io::BufWriter::new(f);
    let mut buf = [0; 1024 * 1024];
    let value = Arc::new(AtomicU64::new(0));
    let total = resp.content_length().unwrap();
    debug!("content-length: {}", total);
    let value_1 = Arc::clone(&value);
    let is_stop = Arc::new(AtomicBool::new(false));
    let is_stop_1 = Arc::clone(&is_stop);
    thread::spawn(move || loop {
        if is_stop_1.load(Ordering::Relaxed) {
            return;
        }
        thread::sleep(Duration::from_millis(300));
        let _ = tx.send(UpdateStatus::Processing {
            progress: Progress {
                value: value_1.load(Ordering::Relaxed),
                total,
            },
        });
    });
    loop {
        let n = resp
            .read(&mut buf)
            .map_err(|_| Error::ReadDownloadContentFailed)?;
        if n == 0 {
            break;
        }
        writer
            .write(&buf[..n])
            .map_err(|_| Error::WriteDownloadContentFailed)?;

        value.fetch_add(n as u64, Ordering::Relaxed);
    }
    writer.flush().map_err(|_| Error::CreateFileFailed)?;

    is_stop.store(true, Ordering::Relaxed);
    Ok(())
}

pub fn restart(version_manager: Arc<Mutex<VersionManager>>) {
    let version_manager_g = version_manager.lock().unwrap();
    let exe_path = version_manager_g.exe_path.clone();
    drop(version_manager_g);

    let mut c = Command::new(exe_path);
    let r = c.spawn();
    if let Err(e) = r {
        warn!("start new version program failed, err: {}", e);
        return;
    }
    process::exit(0);
}
