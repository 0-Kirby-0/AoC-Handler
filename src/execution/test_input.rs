pub enum TestInput {
    None,
    Empty,
    Input(String),
}

impl From<()> for TestInput {
    fn from(_value: ()) -> Self {
        Self::None
    }
}

impl From<&str> for TestInput {
    fn from(value: &str) -> Self {
        if value.is_empty() {
            Self::Empty
        } else {
            Self::Input(value.to_owned())
        }
    }
}

impl From<String> for TestInput {
    fn from(value: String) -> Self {
        if value.is_empty() {
            Self::Empty
        } else {
            Self::Input(value)
        }
    }
}
