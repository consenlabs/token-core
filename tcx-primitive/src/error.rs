pub struct Error {
    pub code: String,
    pub message: String,
}

impl Error {
    pub fn new(code: String, message: String) -> Error {
        Error { code, message }
    }
}
