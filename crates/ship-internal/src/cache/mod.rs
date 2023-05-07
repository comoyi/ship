use log::{debug, warn};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Write;
use std::path::Path;
use std::time::UNIX_EPOCH;
use std::{fs, io, time};
use util::filepath;

#[derive(Debug)]
pub enum CacheError {
    GetProgramDirPathFailed,
    GetCacheDirPathFailed,
    GetUpdateCacheDirPathFailed,
    CreateCacheDirFailed,
    SaveCacheFileFailed,
    CachePathError,
    SaveCacheDbFailed,
    SerializeCacheInfoFailed,
    CalcHashFailed,
    GetCacheInfoFailed,
    ConvertPathToStrFailed,
}

pub fn get_cache_file(hash_sum: &str) -> Option<CacheFile> {
    let cache_info = get_cache_info().ok()?;
    let f = cache_info.files.get(hash_sum)?;
    return Some(f.clone());
}

pub fn add_to_cache<P: AsRef<Path>>(src_path: P, app_id: u64) -> Result<(), CacheError> {
    let cache_dir_path =
        get_update_cache_dir_path().map_err(|_| CacheError::GetUpdateCacheDirPathFailed)?;
    fs::create_dir_all(&cache_dir_path).map_err(|e| {
        warn!("create cache dir failed, err: {}", e);
        return CacheError::CreateCacheDirFailed;
    })?;
    let d = util::time::format_timestamp_to_date(chrono::Utc::now().timestamp());
    let t = time::SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let cache_name = format!("{}", t);
    let dst_dir_path = Path::new(&cache_dir_path).join(app_id.to_string()).join(d);
    let r = fs::create_dir_all(&dst_dir_path);
    if let Err(e) = r {
        warn!("create cache dir failed, err: {}", e);
        return Err(CacheError::CreateCacheDirFailed);
    }

    let dst_path = dst_dir_path.join(&cache_name);
    debug!("before copy to cache");
    save_cache_file(&src_path, &dst_path)?;
    debug!("after copy to cache");
    let cache_rel_path = dst_path
        .strip_prefix(&cache_dir_path)
        .map_err(|_| CacheError::CachePathError)?;
    add_to_db(&dst_path, cache_rel_path).map_err(|e| {
        warn!("add to db failed, err: {:?}", e);
        return e;
    })?;
    Ok(())
}

fn save_cache_file<P: AsRef<Path>, Q: AsRef<Path>>(
    src_path: P,
    dst_path: Q,
) -> Result<(), CacheError> {
    // on windows slow ?why
    // fs::copy(&src_path, &dst_path)
    //     .map_err(|e| {
    //     warn!("save cache file failed, err: {}", e);
    //     return CacheError::SaveCacheFileFailed;
    // })?;

    let rf = fs::File::open(&src_path).map_err(|_| CacheError::SaveCacheFileFailed)?;
    let mut br = io::BufReader::with_capacity(1024 * 1024, rf);
    let wf = fs::File::create(&dst_path).map_err(|_| CacheError::SaveCacheFileFailed)?;
    let mut bw = io::BufWriter::with_capacity(1024 * 1024, wf);
    io::copy(&mut br, &mut bw).map_err(|e| {
        warn!("save cache file failed, err: {}", e);
        return CacheError::SaveCacheFileFailed;
    })?;
    bw.flush().map_err(|_| CacheError::SaveCacheFileFailed)?;
    Ok(())
}

fn add_to_db<P: AsRef<Path>>(dst_path: P, rel_path: &Path) -> Result<(), CacheError> {
    let hash_sum = util::hash::md5::md5_file(&dst_path).map_err(|_| CacheError::CalcHashFailed)?;
    let cache_file = CacheFile::new(
        rel_path
            .to_str()
            .ok_or(CacheError::ConvertPathToStrFailed)?
            .to_string(),
        &hash_sum,
    );
    let mut cache_info = get_cache_info()?;
    cache_info.files.insert(hash_sum, cache_file);
    save_cache_info(cache_info)?;
    Ok(())
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CacheFile {
    pub relative_path: String,
    pub hash: String,
}

impl CacheFile {
    fn new(rel_path: String, hash_sum: &str) -> Self {
        Self {
            relative_path: rel_path,
            hash: hash_sum.clone().to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct CacheInfo {
    files: HashMap<String, CacheFile>,
}

impl Default for CacheInfo {
    fn default() -> Self {
        Self {
            files: Default::default(),
        }
    }
}

fn get_cache_info() -> Result<CacheInfo, CacheError> {
    let p = get_cache_db_file()?;
    if !Path::new(&p).exists() {
        return Ok(CacheInfo::default());
    }
    let d = fs::read_to_string(&p).map_err(|_| CacheError::GetCacheInfoFailed)?;
    let ci = serde_json::from_str::<CacheInfo>(&d).map_err(|e| {
        warn!("deserialize failed, data: {}, err: {}", &d, e);
        return CacheError::GetCacheInfoFailed;
    })?;
    Ok(ci)
}

fn save_cache_info(cache_info: CacheInfo) -> Result<(), CacheError> {
    let p = get_cache_db_file()?;
    let j = serde_json::to_string(&cache_info).map_err(|_| CacheError::SerializeCacheInfoFailed)?;

    fs::write(p, j).map_err(|_| CacheError::SaveCacheDbFailed)?;
    Ok(())
}

pub fn get_update_cache_dir_path() -> Result<String, CacheError> {
    let cache_dir_path = get_cache_dir_path()?;
    let dir_path = Path::new(&cache_dir_path).join("update");
    Ok(dir_path
        .to_str()
        .ok_or(CacheError::ConvertPathToStrFailed)?
        .to_string())
}

pub fn get_cache_dir_path() -> Result<String, CacheError> {
    let program_dir_path =
        filepath::get_exe_dir().map_err(|_| CacheError::GetProgramDirPathFailed)?;
    let cache_dir_path = Path::new(&program_dir_path).join(".cache");
    Ok(cache_dir_path
        .to_str()
        .ok_or(CacheError::ConvertPathToStrFailed)?
        .to_string())
}

fn get_cache_db_file() -> Result<String, CacheError> {
    let cache_dir_path = get_cache_dir_path().map_err(|_| CacheError::GetCacheDirPathFailed)?;
    let file_name = "update-cache-db";
    let p = Path::new(&cache_dir_path).join(file_name);
    Ok(p.to_str()
        .ok_or(CacheError::ConvertPathToStrFailed)?
        .to_string())
}
