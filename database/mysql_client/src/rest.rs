pub mod rest {
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    pub struct Sensor {
        #[serde(rename = "type")]
        pub sensor_type: String,
        pub id: i32,
        pub value: i32,
        pub trend: i32,
        pub state: i32,
        #[serde(rename = "elapsedTimeS")]
        pub elapsed_time_s: i32,
    }

    #[derive(Debug, Deserialize)]
    pub struct MultiSensor {
        pub sensors: Vec<Sensor>,
    }

    #[derive(Debug, Deserialize)]
    pub struct Root {
        pub multiSensor: MultiSensor,
    }
}