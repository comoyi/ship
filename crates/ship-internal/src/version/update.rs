use crate::version::version_manage::VersionManager;
use crate::{request, version};
use log::{debug, info, warn};
use std::io::{Read, Write};
use std::path::Path;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::mpsc::Sender;
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;
use std::{env, fs, io, thread};

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
}

pub fn update_new_version(version_manager: Arc<Mutex<VersionManager>>) {
    thread::spawn(move || {
        let mut version_manager_g = version_manager.lock().unwrap();
        version_manager_g.is_updating = true;
        drop(version_manager_g);

        // let is_cancel = Arc::new(AtomicBool::new(false));

        let (tx, rx) = mpsc::channel::<UpdateStatus>();

        let version_manager_1 = Arc::clone(&version_manager);
        thread::spawn(move || loop {
            thread::sleep(Duration::from_millis(300));
            let m_r = rx.recv();
            let m = m_r.unwrap();

            let mut version_manager_g = version_manager_1.lock().unwrap();
            version_manager_g.update_status = m;
            drop(version_manager_g);
        });

        let r = do_update_new_version(tx.clone());
        if let Err(e) = r {
            warn!("update new version failed, err: {:?}", e);
            let mut version_manager_g = version_manager.lock().unwrap();
            version_manager_g.is_updating = false;
            drop(version_manager_g);
            drop(tx);
        } else {
            info!("update new version finished");
        }
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
    do_download(&download_url, &tmp_file_path, tx)?;

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
    let mut value = Arc::new(AtomicU64::new(0));
    let total = resp.content_length().unwrap();
    debug!("content-length: {}", total);
    let value_1 = Arc::clone(&value);
    thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(100));

        tx.send(UpdateStatus::Processing {
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
    Ok(())
}
