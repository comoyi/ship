use crate::data::{ClientFileInfo, FileInfo, Server, ServerFileInfo};
use crate::error::Error;
use crate::scan;
use crate::sync::sync_files;
use log::debug;
use std::path::Path;

pub fn start(id: String, servers: &Vec<Server>) {
    debug!("update, id: {}", id);
    let server_o = get_server_by_id(id.to_string(), servers);
    let server;
    match server_o {
        None => {
            debug!("server not found, id: {}", id);
            return;
        }
        Some(s) => {
            server = s;
        }
    }
    let j_r = serde_json::to_string(&server);
    let j = j_r.unwrap_or("".to_string());
    debug!("found server, id: {}, server: {:?}", id, j);
    let sfi;
    let sfi_r = get_server_file_info(&server);
    match sfi_r {
        Ok(v) => {
            sfi = v;
        }
        Err(_) => {
            debug!("get_server_file_info failed, server: {}", j);
            return;
        }
    }

    let cfi;
    let cfi_r = get_client_file_info(&server);
    match cfi_r {
        Ok(v) => {
            cfi = v;
        }
        Err(_) => {
            debug!("get_client_file_info failed, server: {}", j);
            return;
        }
    }

    let (add_files, changed_files, del_files) = diff_server_client(&sfi, &cfi);
    debug!(
        "sfi: {:?},cfi: {:?}, add_files: {:?}, changed_files: {:?}, del_files: {:?}",
        sfi, cfi, add_files, changed_files, del_files
    );
    print_diff_detail(&sfi, &cfi, &add_files, &changed_files, &del_files);

    sync_files(&add_files,&changed_files,&del_files);
}

fn get_server_file_info(s: &Server) -> Result<ServerFileInfo, Error> {
    let url = get_full_url("/files", s);
    debug!("url: {}", url);
    let resp_r = reqwest::blocking::get(url);
    match resp_r {
        Ok(resp) => {
            let sfi_r = resp.json::<ServerFileInfo>();
            match sfi_r {
                Ok(sfi) => {
                    debug!("{}", serde_json::to_string(&sfi).unwrap());
                    return Ok(sfi);
                }
                Err(_) => {
                    return Err(Error::DeserializeServerFileInfoError);
                }
            }
        }
        Err(_) => {
            return Err(Error::GetServerFileInfoError);
        }
    }
}

fn get_client_file_info(s: &Server) -> Result<ClientFileInfo, Error> {
    let cfi_r = scan::scan(&s.dir);
    cfi_r
}

pub fn get_full_url(u: &str, s: &Server) -> String {
    format!("{}://{}:{}{}", s.protocol, s.host, s.port, u)
}

fn get_server_by_id(id: String, servers: &Vec<Server>) -> Option<&Server> {
    let mut server = None;
    for s in servers {
        if s.id == id {
            server = Some(s);
            break;
        }
    }
    server
}

fn diff_server_client(
    sfi: &ServerFileInfo,
    cfi: &ClientFileInfo,
) -> (Vec<FileInfo>, Vec<FileInfo>, Vec<FileInfo>) {
    let mut del_files: Vec<FileInfo> = vec![];
    let mut add_files: Vec<FileInfo> = vec![];
    let mut changed_files: Vec<FileInfo> = vec![];

    for cf in &cfi.files {
        if !is_in(cf, &sfi.files) {
            del_files.push(cf.clone());
        }
    }
    for sf in &sfi.files {
        if !is_in(sf, &cfi.files) {
            add_files.push(sf.clone());
        }
    }
    for sf in &sfi.files {
        let sf_path = Path::new(&sf.relative_path);
        for cf in &cfi.files {
            let cf_path = Path::new(&cf.relative_path);
            if cf_path.eq(sf_path) {
                if cf.size != sf.size || cf.hash != sf.hash {
                    changed_files.push(sf.clone());
                }
            }
            break;
        }
    }
    (add_files, changed_files, del_files)
}

fn is_in(f: &FileInfo, fs: &Vec<FileInfo>) -> bool {
    let f_path = Path::new(&f.relative_path);
    let mut flag = false;
    for x in fs {
        let x_path = Path::new(&x.relative_path);
        if x_path.eq(f_path) {
            flag = true;
            break;
        }
    }
    if flag {
        return true;
    }
    false
}

fn print_diff_detail(
    sfi: &ServerFileInfo,
    cfi: &ClientFileInfo,
    add_files: &Vec<FileInfo>,
    changed_files: &Vec<FileInfo>,
    del_files: &Vec<FileInfo>,
) {
    print_file_info(&sfi.files, "server");
    print_file_info(&cfi.files, "client");
    print_file_info(&add_files, "add_files");
    print_file_info(&changed_files, "changed_files");
    print_file_info(&del_files, "del_files");
}

fn print_file_info(fi: &Vec<FileInfo>, s: &str) {
    debug!("------- {} -------", s);
    for f in fi {
        debug!(
            "type: {}, hash: {:32}, size: {:10}, rel_path: {}",
            f.file_type.to_formatted_string(),
            f.hash,
            f.size,
            f.relative_path
        );
    }
    debug!("------- {} -------", s);
}
