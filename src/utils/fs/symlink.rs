use std::path::Path;
use std::{fs, io, os};

#[cfg(not(windows))]
use std::os::unix;

#[cfg(windows)]
use std::os::windows;

#[cfg(not(windows))]
pub fn symlink_file<P: AsRef<Path>, Q: AsRef<Path>>(original: P, link: Q) -> io::Result<()> {
    symlink(original, link)
}

#[cfg(windows)]
pub fn symlink_file<P: AsRef<Path>, Q: AsRef<Path>>(original: P, link: Q) -> io::Result<()> {
    windows::fs::symlink_file(original.as_ref(), link.as_ref())
}

#[cfg(not(windows))]
pub fn symlink_dir<P: AsRef<Path>, Q: AsRef<Path>>(original: P, link: Q) -> io::Result<()> {
    symlink(original, link)
}

#[cfg(windows)]
pub fn symlink_dir<P: AsRef<Path>, Q: AsRef<Path>>(original: P, link: Q) -> io::Result<()> {
    windows::fs::symlink_dir(original.as_ref(), link.as_ref())
}

#[cfg(not(windows))]
fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(original: P, link: Q) -> io::Result<()> {
    unix::fs::symlink(original.as_ref(), link.as_ref())
}
