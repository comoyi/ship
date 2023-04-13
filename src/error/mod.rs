#[derive(Debug)]
pub enum Error {
    QueryError,
    ReadBodyError,
    DecodeError,

    ScanFinalDataError,

    GetServerFileInfoError,
    DeserializeServerFileInfoError,
    ScanError,
    ScanPathNotExitError,
    GetClientFileInfoError,
    CalcHashError,
}

pub enum SyncError {
    PathInvalid,

    CheckExistsFailed,
    UnknownFileType,
    CreateDirFailed,
    CreateSymlinkFailed,
    DeleteFailed,

    DownloadFailed,
    ReadDownloadContentFailed,
    WriteDownloadContentFailed,
    CreateFileFailed,

    CheckSyncedFileError,
    SyncedFileHashError,

    SyncFromCacheError,
    AddToCacheFailed,
}
