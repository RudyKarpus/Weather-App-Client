use std::ops::Deref;

#[derive(Debug, Clone, Default)]
#[repr(transparent)]
pub struct Token(String);

impl Token {
    #[allow(dead_code, reason = "Will be used in future")]
    pub fn new(data: String) -> Self {
        Self(data)
    }
}

impl Deref for Token {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
