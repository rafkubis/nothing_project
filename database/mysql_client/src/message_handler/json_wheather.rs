use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct DtClouds {
    pub dt: u32,
    pub cloud: u32,
}

#[derive(Debug, Deserialize)]
pub struct Wheather {
    pub data: Vec<DtClouds>,
}

#[derive(Debug, Deserialize)]
pub struct Root {
    pub wheather: Vec<DtClouds>,
}
