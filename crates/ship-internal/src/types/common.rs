use serde_repr::Deserialize_repr;

#[derive(Deserialize_repr, Debug)]
#[repr(i8)]
pub enum ScanStatus {
    Wait = 10,
    Scanning = 20,
    Failed = 30,
    Completed = 40,
}

#[derive(Deserialize_repr, Debug)]
#[repr(i8)]
pub enum FileType {
    Unknown = 0,
    File = 1,
    Dir = 2,
    Symlink = 4,
}
