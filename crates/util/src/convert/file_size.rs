pub fn simple_format(size: u64) -> String {
    if size < 1024 {
        format!("{}B", size)
    } else if size < 1024 * 1024 {
        format!("{}K", (size / 1024))
    } else if size < 1024 * 1024 * 1024 {
        format!("{}M", (size / (1024 * 1024)))
    } else {
        format!("{}G", (size / (1024 * 1024 * 1024)))
    }
}
