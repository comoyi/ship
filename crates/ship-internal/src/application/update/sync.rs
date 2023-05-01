use crate::types::common::{DataNode, FileInfo};
use log::debug;

#[derive(Debug)]
pub struct SyncTask {
    pub sync_type: SyncTaskType,
    pub file_info: FileInfo,
    pub base_path: String,
    pub data_nodes: Vec<DataNode>,
}

impl SyncTask {
    pub fn new(
        sync_type: SyncTaskType,
        file_info: FileInfo,
        base_path: String,
        data_nodes: Vec<DataNode>,
    ) -> Self {
        Self {
            sync_type,
            file_info,
            base_path,
            data_nodes,
        }
    }
}

#[derive(Debug)]
pub enum SyncTaskType {
    Create,
    Update,
    Delete,
}

pub fn handle_task(sync_task: SyncTask) {
    debug!("SyncTask: {:?}", sync_task);
}
