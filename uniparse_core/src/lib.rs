pub trait ParsedFile:
    std::fmt::Debug
    + Clone
    + PartialEq
    + std::fmt::Display
    + std::str::FromStr
    + serde::Serialize
    + serde::de::DeserializeOwned
{
    fn parse_str(src: &str) -> Result<Self, String>
    where
        Self: Sized;

    fn to_string_pretty(&self) -> String;
}
