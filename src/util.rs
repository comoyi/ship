pub fn md5sum(s: &str) -> String {
    let s = md5::compute(s);
    format!("{:x}", s)
}
