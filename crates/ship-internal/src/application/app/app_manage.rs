use crate::application::app::app_server::{AppServer, AppServerInfo, AppServers};
use crate::application::app::{App, AppManager, Apps};
use crate::request;
use crate::request::app_server::announcement::AnnouncementVo;
use crate::types::banner::Banner;
use image::ImageFormat;
use log::warn;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{fs, io, thread};
use util::filepath;
use util::hash::md5;

pub enum Error {
    GetAppsFailed,
    GetAppServersFailed,
}

pub fn start(app_manager: Arc<Mutex<AppManager>>) -> Result<(), Error> {
    let apps_vo = request::get_apps::get_apps().map_err(|_| Error::GetAppsFailed)?;
    let mut apps = Apps::new();

    for app_tmp in &apps_vo.apps {
        let app_servers_vo = request::get_app_servers::get_app_servers(app_tmp.id)
            .map_err(|_| Error::GetAppServersFailed)?;
        let mut app_servers = AppServers::new();

        let mut selected_app_server_id = None;
        let mut sort_vec: Vec<_> = app_servers_vo.servers.iter().collect();
        sort_vec.sort_by(|a, b| b.priority.cmp(&a.priority));
        if let Some(app) = sort_vec.first() {
            selected_app_server_id = Some(app.id);
        }
        for app_server_tmp in app_servers_vo.servers {
            let app_server = AppServer::new(
                app_server_tmp.id,
                app_server_tmp.app_id,
                &app_server_tmp.name,
                app_server_tmp.address,
                &app_server_tmp.description,
                app_server_tmp.priority,
            );
            app_servers.insert(app_server.id, app_server);
        }
        let app = App {
            id: app_tmp.id,
            name: app_tmp.name.clone(),
            code: app_tmp.code.clone(),
            dir_name: app_tmp.dir_name.clone(),
            priority: app_tmp.priority,
            launch: app_tmp.launch.clone(),
            app_server_info: AppServerInfo::new(app_servers),
            selected_app_server_id,
        };
        apps.insert(app.id, app);
    }

    let mut sort_vec: Vec<_> = apps_vo.apps.iter().collect();
    sort_vec.sort_by(|a, b| b.priority.cmp(&a.priority));

    let mut app_manager_g = app_manager.lock().unwrap();
    app_manager_g.apps = apps;
    if let Some(app) = sort_vec.first() {
        app_manager_g.selected_app_id = Some(app.id);
    }
    drop(app_manager_g);

    let app_manager_ptr = Arc::clone(&app_manager);
    thread::spawn(move || {
        loop_refresh_announcement(app_manager_ptr);
    });

    let app_manager_ptr = Arc::clone(&app_manager);
    thread::spawn(move || {
        loop_refresh_banner(app_manager_ptr);
    });

    Ok(())
}

fn loop_refresh_announcement(app_manager: Arc<Mutex<AppManager>>) {
    loop {
        refresh_announcement(Arc::clone(&app_manager));
        thread::sleep(Duration::from_secs(60));
    }
}

pub fn refresh_announcement(app_manager: Arc<Mutex<AppManager>>) {
    thread::spawn(move || {
        let (app_server_id, app_id, address) =
            get_current_app_server_info(Arc::clone(&app_manager));

        if let Some(address) = address {
            let announcement_r = request::app_server::announcement::get_announcement(&address);
            match announcement_r {
                Ok(announcement_vo) => {
                    set_announcement(
                        app_server_id,
                        app_id,
                        announcement_vo,
                        Arc::clone(&app_manager),
                    );
                }
                Err(_) => {
                    warn!("get announcement failed");
                }
            }
        }
    });
}

fn set_announcement(
    app_server_id: u64,
    app_id: u64,
    announcement_vo: AnnouncementVo,
    app_manager: Arc<Mutex<AppManager>>,
) {
    let mut app_manager_g = app_manager.lock().unwrap();

    if let Some(app) = app_manager_g.apps.get_mut(&app_id) {
        if let Some(app_server) = app.app_server_info.servers.get_mut(&app_server_id) {
            app_server.announcement.content = announcement_vo.content;
        }
    }

    drop(app_manager_g);
}

fn loop_refresh_banner(app_manager: Arc<Mutex<AppManager>>) {
    loop {
        refresh_banner(Arc::clone(&app_manager));
        thread::sleep(Duration::from_secs(120));
    }
}

pub fn refresh_banner(app_manager: Arc<Mutex<AppManager>>) {
    thread::spawn(move || {
        let (app_server_id, app_id, address) =
            get_current_app_server_info(Arc::clone(&app_manager));

        if let Some(address) = address {
            let banner_r = request::app_server::banner::get_banner(&address);
            match banner_r {
                Ok(banner_vo) => {
                    let mut banners = vec![];
                    let max = 10;
                    let mut count = 0;
                    for x in banner_vo.banners {
                        count += 1;
                        if count > max {
                            break;
                        }
                        let mut banner = Banner::new(&x.image_url, &x.description);
                        let image_path = get_local_image_path(&x.image_url, app_server_id, app_id)
                            .unwrap_or("".to_string());
                        banner.image_path = image_path.clone();
                        if let Ok(mut f) = fs::File::open(&image_path) {
                            let img = image::load(io::BufReader::new(&f), ImageFormat::WebP)
                                .or_else(|_| {
                                    f.seek(SeekFrom::Start(0))?;
                                    image::load(io::BufReader::new(&f), ImageFormat::Png)
                                })
                                .or_else(|_| {
                                    f.seek(SeekFrom::Start(0))?;
                                    image::load(io::BufReader::new(&f), ImageFormat::Jpeg)
                                });
                            if img.is_err() {
                                continue;
                            }
                            let image_data = img.unwrap_or_default().thumbnail(360, 360).to_rgba8();
                            banner.image_data = image_data;
                        }

                        banners.push(banner);
                    }
                    set_banner(app_server_id, app_id, banners, Arc::clone(&app_manager));
                }
                Err(_) => {
                    warn!("get banner failed");
                }
            }
        }
    });
}

fn set_banner(
    app_server_id: u64,
    app_id: u64,
    banners: Vec<Banner>,
    app_manager: Arc<Mutex<AppManager>>,
) {
    let mut app_manager_g = app_manager.lock().unwrap();

    if let Some(app) = app_manager_g.apps.get_mut(&app_id) {
        if let Some(app_server) = app.app_server_info.servers.get_mut(&app_server_id) {
            app_server.banners = banners;
        }
    }

    drop(app_manager_g);
}

fn get_current_app_server_info(app_manager: Arc<Mutex<AppManager>>) -> (u64, u64, Option<String>) {
    let mut address = None;
    let mut app_id = 0;
    let mut app_server_id = 0;
    let mut app_manager_g = app_manager.lock().unwrap();
    if let Some(selected_app_id) = app_manager_g.selected_app_id {
        app_id = selected_app_id;
        if let Some(app) = app_manager_g.apps.get_mut(&selected_app_id) {
            if let Some(selected_app_server_id) = app.selected_app_server_id {
                app_server_id = selected_app_server_id;
                if let Some(app_server) =
                    app.app_server_info.servers.get_mut(&selected_app_server_id)
                {
                    address = Some(app_server.address.to_address_string());
                }
            }
        }
    }
    drop(app_manager_g);
    (app_server_id, app_id, address)
}

#[derive(Debug)]
enum DownloadImageError {
    GetBasePathFailed,
    CreateDirFailed,
    DownloadFailed,
    CreateFileFailed,
    ReadDownloadContentFailed,
    WriteDownloadContentFailed,
    RenameFileFailed,
    ConvertPathToStringFailed,
}

fn get_local_image_path(
    url: &str,
    app_server_id: u64,
    app_id: u64,
) -> Result<String, DownloadImageError> {
    let program_dir_path =
        filepath::get_exe_dir().map_err(|_| DownloadImageError::GetBasePathFailed)?;

    let banner_base_path = Path::new(&program_dir_path)
        .join(".cache")
        .join("banner")
        .join(app_id.to_string())
        .join(app_server_id.to_string());
    fs::create_dir_all(&banner_base_path).map_err(|_| DownloadImageError::CreateDirFailed)?;

    // hash by url
    let file_name = md5::md5_string(url);

    // TODO this is tmp fix, windows cant open image without extension
    #[cfg(windows)]
    let file_name = md5::md5_string(url) + ".webp";

    let full_file_path = banner_base_path.join(file_name);

    if full_file_path.try_exists().unwrap_or(false) {
        return Ok(full_file_path
            .to_str()
            .ok_or(DownloadImageError::ConvertPathToStringFailed)?
            .to_string());
    }

    let tmp_file_name = format!("tmp_{}", chrono::Utc::now().timestamp());
    let tmp_file_path = banner_base_path.join(tmp_file_name);
    let mut resp = reqwest::blocking::get(url).map_err(|_| DownloadImageError::DownloadFailed)?;
    let f = fs::File::create(&tmp_file_path).map_err(|_| DownloadImageError::CreateFileFailed)?;
    let mut writer = io::BufWriter::new(f);
    let mut buf = [0; 1024 * 1024];
    loop {
        let n = resp
            .read(&mut buf)
            .map_err(|_| DownloadImageError::ReadDownloadContentFailed)?;
        if n == 0 {
            break;
        }
        writer
            .write(&buf[..n])
            .map_err(|_| DownloadImageError::WriteDownloadContentFailed)?;
    }
    writer
        .flush()
        .map_err(|_| DownloadImageError::CreateFileFailed)?;
    fs::rename(&tmp_file_path, &full_file_path)
        .map_err(|_| DownloadImageError::RenameFileFailed)?;
    Ok(full_file_path
        .to_str()
        .ok_or(DownloadImageError::ConvertPathToStringFailed)?
        .to_string())
}
