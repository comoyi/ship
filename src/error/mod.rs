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
    CheckExistsFailed,
    UnknownFileType,
    CreateDirFailed,
    CreateSymlinkFailed,
    DeleteFailed,

    DownloadFailed,
    ReadDownloadContentFailed,
    CreateFileFailed,
}
