use time::{OffsetDateTime, Time};

pub struct TimeSegment {
    pub start_time: Option<OffsetDateTime>,
    pub end_time: Option<OffsetDateTime>,
    pub start_time_hour_field: String,
    pub start_time_minute_field: String,
    pub end_time_hour_field: String,
    pub end_time_minute_field: String,
    pub hours_total: f64,
}

impl TimeSegment {
    pub fn new(is_rounding_on: bool, minute_rounding_scale: f32) -> TimeSegment {
        let mut _self = TimeSegment {
            start_time: None,
            end_time: None,
            start_time_hour_field: "".to_owned(),
            start_time_minute_field: "".to_owned(),
            end_time_hour_field: "".to_owned(),
            end_time_minute_field: "".to_owned(),
            hours_total: 0f64,
        };

        let mut current_time = OffsetDateTime::now_local().ok();

        if is_rounding_on {
            let current_time_hms = current_time.unwrap().to_hms();
            let rounded_time = _self.round_time(
                (current_time_hms.0, current_time_hms.1), minute_rounding_scale);
            let offset_rounded_time = current_time.unwrap().replace_time(Time::from_hms(rounded_time.0, rounded_time.1, 0).unwrap());
            current_time = Some(offset_rounded_time);
        }
        _self.start_time = current_time;
        let start_time_formatted = format_time_hour_minute(_self.start_time.unwrap());
        _self.start_time_hour_field = start_time_formatted.0;
        _self.start_time_minute_field = start_time_formatted.1;

        _self
    }

    pub fn record_end_time(&mut self, is_rounding_on: bool, minute_rounding_scale: f32) {
        let mut current_time = OffsetDateTime::now_local().ok();

        if is_rounding_on {
            let current_time_hms = current_time.unwrap().to_hms();
            let rounded_time = self.round_time(
                (current_time_hms.0, current_time_hms.1), minute_rounding_scale);
            let offset_rounded_time = current_time.unwrap().replace_time(Time::from_hms(rounded_time.0, rounded_time.1, 0).unwrap());
            current_time = Some(offset_rounded_time);
        }

        self.end_time = current_time;
        let end_time_formatted = format_time_hour_minute(self.end_time.unwrap());
        self.end_time_hour_field = end_time_formatted.0;
        self.end_time_minute_field = end_time_formatted.1;

        self.calculate_total_hours();
    }

    pub fn calculate_total_hours(&mut self) {
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
            if rounded_time.0 > 23 {
                rounded_time.0 = 0;
            }
            rounded_time.1 = minutes % 60;
        } else {
            rounded_time.1 = minutes;
        }

        rounded_time
    }
}

fn format_time_hour_minute(time_stamp: OffsetDateTime) -> (String, String) {
    (time_stamp.to_hms().0.to_string(), time_stamp.to_hms().1.to_string())
}