// Struct som representerar en siffra, den siffran kan antingen vara ett värde eller ett x-värde
#[derive(Debug, Clone, Copy)]
pub enum Int {
    Value(f64),
    Unknown,
}

impl From<&str> for Int {
    fn from(value: &str) -> Self {
        if value == "x" {
            Self::Unknown
        } else {
            Self::Value(value.parse::<f64>().unwrap())
        }
    }
}
