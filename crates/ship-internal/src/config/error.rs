#[derive(Debug)]
pub enum Error {
    GetExeDirFailed,
    BuildConfigFailed,
    DeserializeConfigFailed,
}
