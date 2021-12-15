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

    pub fn human_readable_output(&self) -> String {
        const KIBIBYTE: u64 = 1024;
        const MEBIBYTE: u64 = 1_048_576;
        const GIBIBYTE: u64 = 1_073_741_824;
        const TEBIBYTE: u64 = 1_099_511_627_776;
        const PEBIBYTE: u64 = 1_125_899_906_842_624;
        const EXBIBYTE: u64 = 1_152_921_504_606_846_976;

        let (size, symbol) = match self.bytes {
            size if size < KIBIBYTE => (self.bytes as f64, "B"),
            size if size < MEBIBYTE => (self.bytes as f64 / KIBIBYTE as f64, "KiB"),
            size if size < GIBIBYTE => (self.bytes as f64 / MEBIBYTE as f64, "MiB"),
            size if size < TEBIBYTE => (self.bytes as f64 / GIBIBYTE as f64, "GiB"),
            size if size < PEBIBYTE => (self.bytes as f64 / TEBIBYTE as f64, "TiB"),
            size if size < EXBIBYTE => (self.bytes as f64 / PEBIBYTE as f64, "PiB"),
            _ => (self.bytes as f64 / EXBIBYTE as f64, "EiB"),
        };

        format!("{:.1}{}", size, symbol)
    }
}

impl fmt::Display for FileSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.human_readable_output())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_bytes() {
        assert_eq!(FileSize::new(1024).bytes(), 1024);
    }

    #[test]
    fn test_human_readable_output() {
        assert_eq!(FileSize::new(1024).human_readable_output(), "1.0KiB");
        assert_eq!(FileSize::new(1_048_576).human_readable_output(), "1.0MiB");
        assert_eq!(
            FileSize::new(1_073_741_824).human_readable_output(),
            "1.0GiB"
        );
        assert_eq!(
            FileSize::new(1_099_511_627_776).human_readable_output(),
            "1.0TiB"
        );
        assert_eq!(
            FileSize::new(1_125_899_906_842_624).human_readable_output(),
            "1.0PiB"
        );
        assert_eq!(
            FileSize::new(1_152_921_504_606_846_976).human_readable_output(),
            "1.0EiB"
        );
    }
}
