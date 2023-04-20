#[derive(Debug)]
pub struct LoxError {
    line: usize,
    message: String,
}
impl LoxError {
    pub fn report(&self, loc: String) {
        eprintln!("[line {}] Error {}: {}", self.line, loc, self.message);
    }

    pub fn error(line: usize, message: String) -> LoxError{
        LoxError { line, message }
    }
}
