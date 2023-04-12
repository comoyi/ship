use std::path::Path;
use std::{fs, io, os};

// TODO
pub fn symlink_file<P: AsRef<Path>, Q: AsRef<Path>>(original: P, link: Q) -> io::Result<()> {
    Ok(())
}

// TODO
pub fn symlink_dir<P: AsRef<Path>, Q: AsRef<Path>>(original: P, link: Q) -> io::Result<()> {
    Ok(())
}
