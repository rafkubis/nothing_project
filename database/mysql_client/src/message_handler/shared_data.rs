use super::json_wheather;

pub struct Data {
    pub clouds_forecast: Vec<json_wheather::DtClouds>,
}

impl Data {
    pub fn new() -> Self {
        Data {
            clouds_forecast: vec![],
        }
    }
}
