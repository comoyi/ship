use log::warn;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::time::UNIX_EPOCH;
use std::{fs, time};
use util::filepath;

#[derive(Debug)]
pub enum CacheError {
    GetProgramDirPathFailed,
    GetCacheDirPathFailed,
    CreateCacheDirFailed,
    SaveCacheFileFailed,
    CachePathError,
    SaveCacheDbFileFailed,
    SerializeCacheInfoFailed,
    CalcHashFailed,
    GetCacheInfoFailed,
}

pub fn get_cache_file(hash_sum: &str) -> Option<CacheFile> {
    let cache_info_r = get_cache_info();
    match cache_info_r {
        Ok(cache_info) => {
            let f_o = cache_info.files.get(hash_sum);
            match f_o {
                None => {
                    return None;
                }
                Some(f) => {
                    return Some(f.clone());
                }
            }
        }
        Err(_) => {
            return None;
        }
    }
}

pub fn add_to_cache<P: AsRef<Path>>(original_path: P) -> Result<(), CacheError> {
    let cache_dir_path_r = get_cache_dir_path();
    let cache_dir_path = match cache_dir_path_r {
        Ok(p) => p,
        Err(_) => {
            return Err(CacheError::GetCacheDirPathFailed);
        }
    };
    let r = fs::create_dir_all(&cache_dir_path);
    if let Err(e) = r {
        warn!("create cache dir failed, err: {}", e);
        return Err(CacheError::CreateCacheDirFailed);
    }
    let d = util::time::format_timestamp_to_date(chrono::Utc::now().timestamp());
    let t = time::SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let cache_name = format!("{}", t);
    let dst_dir_path = Path::new(&cache_dir_path).join(d);
    let r = fs::create_dir_all(&dst_dir_path);
    if let Err(e) = r {
        warn!("create cache dir failed, err: {}", e);
        return Err(CacheError::CreateCacheDirFailed);
    }

    let dst_path = dst_dir_path.join(&cache_name);
    let r = fs::copy(&original_path, &dst_path);
    match r {
        Ok(_) => {
            let cache_rel_path_r = dst_path.strip_prefix(&cache_dir_path);
            match cache_rel_path_r {
                Ok(cache_rel_path) => {
                    let r = add_to_db(&dst_path, cache_rel_path);
                    match r {
                        Ok(_) => {}
                        Err(e) => {
                            warn!("add to db failed, err: {:?}", e);
                            return Err(e);
                        }
                    }
                }
                Err(e) => {
                    return Err(CacheError::CachePathError);
                }
            }
        }
        Err(e) => {
            warn!("save cache file failed, err: {}", e);
            return Err(CacheError::SaveCacheFileFailed);
        }
    }
    Ok(())
}

fn add_to_db<P: AsRef<Path>>(dst_path: P, rel_path: &Path) -> Result<(), CacheError> {
    let hash_sum_r = util::hash::md5::md5_file(dst_path);
    match hash_sum_r {
        Ok(hash_sum) => {
            let cache_file = CacheFile::new(rel_path.to_str().unwrap().to_string(), &hash_sum);
            let mut cache_info_r = get_cache_info();
            match cache_info_r {
                Ok(mut cache_info) => {
                    cache_info.files.insert(hash_sum, cache_file);
                    let r = save_cache_info(cache_info);
                    match r {
                        Ok(_) => {}
                        Err(e) => {
                            return Err(e);
                        }
                    }
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Err(_) => {
            return Err(CacheError::CalcHashFailed);
        }
    }
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
    let p_r = get_cache_db_file();
    let p;
    match p_r {
        Ok(x) => {
            p = x;
        }
        Err(e) => {
            return Err(e);
        }
    }
    if !Path::new(&p).exists() {
        return Ok(CacheInfo::default());
    }
    let d_r = fs::read_to_string(&p);
    match d_r {
        Ok(d) => {
            let ci_r = serde_json::from_str::<CacheInfo>(&d);
            match ci_r {
                Ok(ci) => Ok(ci),
                Err(e) => {
                    warn!("deserialize failed, data: {}, err: {}", &d, e);
                    return Err(CacheError::GetCacheInfoFailed);
                }
            }
        }
        Err(_) => {
            return Err(CacheError::GetCacheInfoFailed);
        }
    }
}

fn save_cache_info(cache_info: CacheInfo) -> Result<(), CacheError> {
    let p_r = get_cache_db_file();
    let p;
    match p_r {
        Ok(x) => {
            p = x;
        }
        Err(e) => {
            return Err(e);
        }
    }
    let j_r = serde_json::to_string(&cache_info);
    match j_r {
        Ok(j) => {
            let r = fs::write(p, j);
            if let Err(e) = r {
                return Err(CacheError::SaveCacheDbFileFailed);
            }
        }
        Err(_) => {
            return Err(CacheError::SerializeCacheInfoFailed);
        }
    }
    Ok(())
}

pub fn get_cache_dir_path() -> Result<String, CacheError> {
    let program_dir_path_r = filepath::get_exe_dir();
    let cache_dir_path = match program_dir_path_r {
        Ok(p) => Path::new(&p).join(".cache"),
        Err(_) => {
            return Err(CacheError::GetProgramDirPathFailed);
        }
    };
    Ok(cache_dir_path.to_str().unwrap().to_string())
}

fn get_cache_db_file() -> Result<String, CacheError> {
    let cache_dir_path_r = get_cache_dir_path();
    let cache_dir_path = match cache_dir_path_r {
        Ok(p) => p,
        Err(_) => {
            return Err(CacheError::GetCacheDirPathFailed);
        }
    };
    let file_name = "cache-db";
    let p = Path::new(&cache_dir_path).join(file_name);
    Ok(p.to_str().unwrap().to_string())
}
