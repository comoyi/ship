#[derive(Debug)]
pub enum Error {
    BuildRequestError,
    RequestError,
    ReadBodyError,
    DecodeError,
}
