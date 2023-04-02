use log::debug;
use crate::data::FileInfo;

mod sync;
pub fn sync_files(added_files: &Vec<FileInfo>, changed_files: &Vec<FileInfo>, deleted_files: &Vec<FileInfo>) {
    debug!("start sync");
}
