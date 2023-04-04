pub enum Error {
    QueryError,
    ReadBodyError,
    DecodeError,

    ScanFinalDataError,

    GetServerFileInfoError,
    DeserializeServerFileInfoError,
    ScanError,
    GetClientFileInfoError,
    CalcHashError,
}
