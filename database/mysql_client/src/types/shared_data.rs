use crate::types::json_wheather;
use std::sync::Arc;

pub type SharedData = Arc<tokio::sync::RwLock<Data>>;

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
