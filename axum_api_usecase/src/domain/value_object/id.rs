#[allow(dead_code)]
#[derive(Debug)]
pub struct ID {
    value: String,
}
impl ID {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }
}

