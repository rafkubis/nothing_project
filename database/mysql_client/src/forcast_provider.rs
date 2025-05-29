pub use crate::logic;
pub use crate::types;
use chrono::NaiveDateTime;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct ForecastProvider {
    shared_data: Arc<RwLock<types::shared_data::Data>>,
}

impl ForecastProvider {
    pub fn new(shared_data: Arc<RwLock<types::shared_data::Data>>) -> Self {
        Self { shared_data }
    }

    pub async fn get(&self) -> Option<Vec<u32>> {
        let chrono_now = helper::cut_to_hours(chrono::Utc::now().naive_utc())
            .and_utc()
            .timestamp() as u32;
        let forecaset = &self.shared_data.read().await.clouds_forecast;

        helper::get_forecast(forecaset, chrono_now, 3)
    }
}

mod helper {
    use super::*;
    use crate::types;
    use chrono::*;

    pub fn get_forecast(
        forecast: &Vec<types::json_wheather::DtClouds>,
        start_dt: u32,
        cnt: usize,
    ) -> Option<Vec<u32>> {
        let begin = forecast.iter().position(|x| x.dt == start_dt);
        if begin.is_none() || (begin.unwrap() + cnt) > forecast.len() {
            return None;
        }
        let result: Vec<u32> = forecast.as_slice()[begin.unwrap()..begin.unwrap() + cnt]
            .iter()
            .map(|x| x.cloud)
            .collect::<Vec<u32>>();
        Some(result)
    }

    pub fn cut_to_hours(dt: NaiveDateTime) -> NaiveDateTime {
        let only_hours = NaiveTime::from_hms_opt(dt.hour(), 0, 0).unwrap();
        NaiveDateTime::new(dt.date(), only_hours)
    }
    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::types::json_wheather;

        fn get_forcast_data() -> Vec<json_wheather::DtClouds> {
            vec![
                json_wheather::DtClouds {
                    dt: 1744452000,
                    cloud: 10,
                },
                json_wheather::DtClouds {
                    dt: 1744455600,
                    cloud: 20,
                },
                json_wheather::DtClouds {
                    dt: 1744492600,
                    cloud: 30,
                },
                json_wheather::DtClouds {
                    dt: 1744496200,
                    cloud: 40,
                },
                json_wheather::DtClouds {
                    dt: 1744499800,
                    cloud: 50,
                },
                json_wheather::DtClouds {
                    dt: 1744503400,
                    cloud: 60,
                },
            ]
        }

        fn get_test_datetime(seconds: i64) -> u32 {
            cut_to_hours(NaiveDateTime::UNIX_EPOCH + Duration::seconds(seconds))
                .and_utc()
                .timestamp() as u32
        }

        #[test]
        fn test_get_forecast() {
            let dt = get_test_datetime(1744455611);

            let result = get_forecast(get_forcast_data().as_ref(), dt, 3);
            assert_eq!(result.unwrap(), [20, 30, 40]);
        }

        #[test]
        fn test_return_none_when_no_dt_found() {
            let dt = get_test_datetime(1744503401);
            let result = get_forecast(get_forcast_data().as_ref(), dt, 1);
            assert_eq!(result, None);
        }

        #[test]
        fn test_return_none_when_out_of_bounds() {
            let dt = get_test_datetime(1744455611);
            let result = get_forecast(get_forcast_data().as_ref(), dt, 6);
            assert_eq!(result, None);
        }

        #[test]
        fn test_return_forecast_till_end() {
            let dt = get_test_datetime(1744455611);
            let result = get_forecast(get_forcast_data().as_ref(), dt, 5);
            assert_eq!(result.unwrap(), [20, 30, 40, 50, 60]);
        }
    }
}
