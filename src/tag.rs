use crate::time_segment::*;

pub struct Tag {
    pub name: String,
    pub time_segments: Vec<TimeSegment>,
    pub is_active_segment: bool,
    pub total_time: f64,
}

impl Tag {
    pub fn new(name: &str) -> Tag {
        let name = name.to_string();
        let mut _self = Tag {
            name,
            time_segments: Vec::new(),
            is_active_segment: false,
            total_time: 0f64,
        };

        _self
    }

    pub fn clear_session(&mut self) {
        self.time_segments.clear();
        self.is_active_segment = false;
        self.total_time = 0f64;
    }

    pub fn start_time_segment(&mut self, is_rounding_on: bool, minute_rounding_scale: f32) {
        if self.is_active_segment == false {
            let new_segment = TimeSegment::new(is_rounding_on, minute_rounding_scale);
            self.time_segments.push(new_segment);
            self.is_active_segment = true;
        } else {
            println!("Active segment already exists");
        }
    }

    pub fn end_time_segment(&mut self, is_rounding_on: bool, minute_rounding_scale: f32) {
        self.is_active_segment = false;
        self.time_segments
            .last_mut()
            .unwrap()
            .record_end_time(is_rounding_on, minute_rounding_scale);
    }

    pub fn calculate_total(&mut self) {
        let mut running_time = 0f64;

        for time_segment in self.time_segments.iter() {
            running_time += time_segment.hours_total;
        }

        self.total_time = running_time;
    }
}