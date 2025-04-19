use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DtClouds {
    dt: u32,
    cloud: u32,
}

#[derive(Debug, Deserialize)]
pub struct Wheather {
    pub data: Vec<DtClouds>,
}

#[derive(Debug, Deserialize)]
pub struct Root {
    pub wheather: Vec<DtClouds>,
}
