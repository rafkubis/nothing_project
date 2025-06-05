use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Root {
    pub sun: u32,
}
