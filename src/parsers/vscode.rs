use anyhow::Result;

use crate::parsers::parser_trait;

pub struct VSCode;

impl parser_trait::Parser for VSCode {
    fn supported_extension(&self) -> &[&str] {
        &["json"]
    }

    fn parse(&self, file: String) -> Result<Vec<String>> {
        Ok(Vec::new())
    }
}
