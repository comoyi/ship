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

mod test {
    use crate::convert::file_size::simple_format;

    #[test]
    fn test_simple_format() {
        assert_eq!("1B", simple_format(1));
        assert_eq!("1K", simple_format(1024));
        assert_eq!("100K", simple_format(102400));
        assert_eq!("1M", simple_format(1024 * 1024));
        assert_eq!("1G", simple_format(1024 * 1024 * 1024));
    }
}
