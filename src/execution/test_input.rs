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
            Self::Input(dedent(value))
        }
    }
}

impl From<String> for TestInput {
    fn from(value: String) -> Self {
        if value.is_empty() {
            Self::Empty
        } else {
            Self::Input(dedent(&value))
        }
    }
}

fn dedent(input: &str) -> String {
    //? I'm unaware of any AoC input that actually relies on leading whitespace anywhere
    //? If something like that arises, I will add proper dedentation a la indoc
    input.lines().map(str::trim_start).collect()
}
