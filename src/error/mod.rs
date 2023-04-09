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
