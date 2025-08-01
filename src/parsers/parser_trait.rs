use anyhow::Result;

pub trait Parser {
    fn supported_extension(&self) -> &[&str];
    fn parse(&self, file: String) -> Result<Vec<String>>;
}
