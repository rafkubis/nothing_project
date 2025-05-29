pub fn should_stop(current_temperature: f32, current_sun: i64, forecast: Vec<u32>) -> bool {
    if current_temperature > 20.0
        && ((current_sun > 50 && forecast[0] > 50) || (forecast[1] > 50 && forecast[2] > 50))
    {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_pass() {
        let current_temperature = 21.0;
        let current_sun = 60;
        let forecast = vec![60, 70, 80];
        assert_eq!(
            should_stop(current_temperature, current_sun, forecast),
            true
        );
    }
}
