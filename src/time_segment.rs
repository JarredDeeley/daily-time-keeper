use time::{OffsetDateTime, Time};

pub struct TimeSegment {
    pub start_time: Option<OffsetDateTime>,
    pub end_time: Option<OffsetDateTime>,
    pub hours_total: f64,
}

impl TimeSegment {
    pub fn new(is_rounding_on: bool, minute_rounding_scale: f32) -> TimeSegment {
        let mut _self = TimeSegment {
            start_time: None,
            end_time: None,
            hours_total: 0f64,
        };

        let current_time = OffsetDateTime::now_local().ok();

        if is_rounding_on {
            let rounded_time = _self.round_time(
                (current_time.unwrap().hour(), current_time.unwrap().minute()), minute_rounding_scale);
            let current_time =
                Time::from_hms(rounded_time.0, rounded_time.1, 0);
        }
        _self.start_time = current_time;

        _self
    }

    pub fn record_end_time(&mut self, is_rounding_on: bool, minute_rounding_scale: f32) {
        let current_time = OffsetDateTime::now_local().ok();

        if is_rounding_on {
            let rounded_time = self.round_time(
                (current_time.unwrap().hour(), current_time.unwrap().minute()), minute_rounding_scale);
            let current_time =
                Time::from_hms(rounded_time.0, rounded_time.1, 0);
        }

        self.end_time = current_time;
        self.calculate_total_hours();
    }

    fn calculate_total_hours(&mut self) {
        let time_duration = self.end_time.unwrap() - self.start_time.unwrap();
        self.hours_total = time_duration.as_seconds_f64() / 3600f64;
    }

    pub fn round_time(&self, time_to_be_rounded: (u8, u8), minute_rounding_scale: f32) -> (u8, u8) {
        let minute_accuracy = (60.0 * minute_rounding_scale).floor();
        let mut rounded_time = time_to_be_rounded;

        let mut minutes = time_to_be_rounded.1;
        minutes = ((minutes as f32 / minute_accuracy + 0.5).floor() * minute_accuracy) as u8;

        if minutes >= 60 {
            rounded_time.0 += 1;
            if rounded_time.0 >= 23 {
                rounded_time.0 = 0;
            }
            rounded_time.1 = minutes % 60;
        } else {
            rounded_time.1 = minutes;
        }

        rounded_time
    }
}