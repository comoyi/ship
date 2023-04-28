pub struct SyncTask {
    pub sync_type: SyncTaskType,
}

impl SyncTask {
    pub fn new(sync_type: SyncTaskType) -> Self {
        Self { sync_type }
    }
}

pub enum SyncTaskType {
    Create,
    Update,
    Delete,
}
