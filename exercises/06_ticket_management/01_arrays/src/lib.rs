// TODO: Flesh out the `WeekTemperatures` struct and its method implementations to pass the tests.
pub struct WeekTemperature {
    weekday: Weekday,
    value: Option<i32>
}

impl WeekTemperature {
    fn new(weekday: Weekday, value: Option<i32>)-> Self {
        Self { weekday: weekday, value: value}
    }
}

pub struct WeekTemperatures {
    week_temperature: [WeekTemperature; 7]
    // TODO
}

#[derive(Debug, PartialEq, Clone)]
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl WeekTemperatures {
    pub fn new() -> Self {
        let temp_week_temperature: [WeekTemperature; 7] = [
            WeekTemperature::new(Weekday::Monday, None),
            WeekTemperature::new(Weekday::Tuesday, None),
            WeekTemperature::new(Weekday::Wednesday, None),
            WeekTemperature::new(Weekday::Thursday, None),
            WeekTemperature::new(Weekday::Friday, None),
            WeekTemperature::new(Weekday::Saturday, None),
            WeekTemperature::new(Weekday::Sunday, None)
        ];
        Self { week_temperature: temp_week_temperature}
    }

    pub fn get_temperature(&self, day: Weekday) -> Option<i32> {
        let found_temperature = self.week_temperature.iter().find(|&r| r.weekday == day);
        if found_temperature.is_none(){
            panic!("something went wrong when finding the date, it should have never happened");
        }
        found_temperature.unwrap().value
    }

    pub fn set_temperature(&mut self, day: Weekday, temperature: i32) {
        let found_temperature = self.week_temperature.iter().position(|r| r.weekday == day);
        if found_temperature.is_none(){
            panic!("something went wrong when finding the date, it should have never happened");
        }
        self.week_temperature[found_temperature.unwrap()] = WeekTemperature::new(day, Some(temperature));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_temperature() {
        let mut week_temperatures = WeekTemperatures::new();

        assert_eq!(week_temperatures.get_temperature(Weekday::Monday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Tuesday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Wednesday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Thursday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Friday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Saturday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Sunday), None);

        week_temperatures.set_temperature(Weekday::Monday, 20);
        assert_eq!(week_temperatures.get_temperature(Weekday::Monday), Some(20));

        week_temperatures.set_temperature(Weekday::Monday, 25);
        assert_eq!(week_temperatures.get_temperature(Weekday::Monday), Some(25));

        week_temperatures.set_temperature(Weekday::Tuesday, 30);
        week_temperatures.set_temperature(Weekday::Wednesday, 35);
        week_temperatures.set_temperature(Weekday::Thursday, 40);
        week_temperatures.set_temperature(Weekday::Friday, 45);
        week_temperatures.set_temperature(Weekday::Saturday, 50);
        week_temperatures.set_temperature(Weekday::Sunday, 55);

        assert_eq!(week_temperatures.get_temperature(Weekday::Monday), Some(25));
        assert_eq!(
            week_temperatures.get_temperature(Weekday::Tuesday),
            Some(30)
        );
        assert_eq!(
            week_temperatures.get_temperature(Weekday::Wednesday),
            Some(35)
        );
        assert_eq!(
            week_temperatures.get_temperature(Weekday::Thursday),
            Some(40)
        );
        assert_eq!(week_temperatures.get_temperature(Weekday::Friday), Some(45));
        assert_eq!(
            week_temperatures.get_temperature(Weekday::Saturday),
            Some(50)
        );
        assert_eq!(week_temperatures.get_temperature(Weekday::Sunday), Some(55));
    }
}
