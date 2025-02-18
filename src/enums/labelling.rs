#[derive(Debug,Clone)]
pub enum Labelling{
    Numeric,
    Alphabetic,
    AlphaNumeric,
}

impl From<&str> for Labelling {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "alphabetic" => Labelling::Alphabetic,
            "alphanumeric" => Labelling::AlphaNumeric,
            _ => Labelling::Numeric, // _ for other scenarios
        }
    }
}

impl<'a> Into<&str> for Labelling {
    fn into(self) -> &'static str {
        match self {
            Labelling::Alphabetic => "alphabetic",
            Labelling::AlphaNumeric => "alphanumeric",
            Labelling::Numeric => "numeric",
        }
    }
}