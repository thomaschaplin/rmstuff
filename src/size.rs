use std::fmt;

#[derive(Debug)]
pub struct FileSize {
    bytes: u64,
}

impl FileSize {
    pub fn new(bytes: u64) -> FileSize {
        FileSize { bytes }
    }

    pub fn bytes(&self) -> u64 {
        self.bytes
    }

    fn human_readable_output(&self) -> String {
        let units = vec!["b", "K", "M", "G"];
        let mut unitless_size = self.bytes as f64;
        let mut divided_times = 0;

        while unitless_size > 1024f64 {
            unitless_size /= 1024f64;
            divided_times += 1;
        }

        format!(
            "{}{}",
            unitless_size.round(),
            units[divided_times].to_string()
        )
    }
}

impl fmt::Display for FileSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.human_readable_output())
    }
}
