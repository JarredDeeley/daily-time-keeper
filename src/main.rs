#![warn(clippy::all, clippy::pedantic)]

use std::borrow::BorrowMut;
use iced::{Element, Sandbox, Settings, Text};
use time::OffsetDateTime;

pub fn main() -> iced::Result {
    TimeManager::run(Settings::default())
}

struct TimeManager;

impl Sandbox for TimeManager {
    type Message = ();

    fn new() -> TimeManager {
        TimeManager
    }

    fn title(&self) -> String {
        String::from("Daily Time Keeper")
    }

    fn update(&mut self, _message: Self::Message) {
        // This application has no interactions
    }

    fn view(&mut self) -> Element<Self::Message> {
        Text::new("Hello, world!").into()
    }
}

struct TagGroup {
    name: String,
    time_segments: Vec<TimeSegment>,
    is_active_segment: bool,
    total_time: f64,
}

impl TagGroup {
    fn new(name: &str) -> TagGroup {
        TagGroup {
            name: String::from(name),
            time_segments: Vec::new(),
            is_active_segment: false,
            total_time: 0f64,
        }
    }

    fn start_time_segment(&mut self) {
        if self.is_active_segment == false {
            let new_segment = TimeSegment::new();
            self.time_segments.push(new_segment);
            self.is_active_segment = true;
        } else {
            println!("Active segment already exists");
        }
    }

    // fn end_time_segment(&mut self) {
    //     self.is_active_segment = false;
    //     self.time_segments
    //         .last()
    //         .expect("No time segments found")
    //         .record_end_time()
    //     ;
    //
    //         // .last().unwrap().record_end_time();
    // }

    fn calculate_total(&mut self) {
    }
}

struct TimeSegment {
    start_time: OffsetDateTime,
    end_time: Option<OffsetDateTime>,
    hours_total: f64,
}

impl TimeSegment {
    fn new() -> TimeSegment {
        TimeSegment {
            start_time: OffsetDateTime::now_local().unwrap(),
            end_time: None,
            hours_total: 0f64,
        }
    }

    fn record_end_time(&mut self) {
        self.end_time = OffsetDateTime::now_local().ok();
        self.calculate_total_hours();
    }

    fn calculate_total_hours(&mut self) {
        let time_duration = self.end_time.unwrap() - self.start_time;
        self.hours_total = time_duration.as_seconds_f64() / 3600f64;
    }
}

#[cfg(test)]
mod tests {
    use std::thread::sleep;
    use std::time::Duration;
    use approx::{assert_relative_eq, relative_eq};
    use super::*;

    #[test]
    fn test_start_segment() {
        let mut test_tag = TagGroup::new("test");
        test_tag.start_time_segment();

        assert_eq!(test_tag.time_segments.len(), 1)
    }

    // #[test]
    // fn test_end_segment() {
    //     let mut test_tag = TagGroup::new("test");
    //     test_tag.start_time_segment();
    //     test_tag.end_time_segment();
    //
    //     assert_eq!(test_tag.is_active_segment, false);
    // }

    // #[test]
    // fn test_calculate_total_hours() {
    //     let mut test_tag = TagGroup::new("test");
    //     test_tag.start_time_segment();
    //     sleep(Duration::from_secs(5));
    //     test_tag.end_time_segment();
    //
    //     assert_eq!(test_tag.time_segments.last().unwrap().hours_total, 0.001388888888888889);
    // }

    #[test]
    fn test_time_segment_record_end_time() {
        let mut test_seg = TimeSegment::new();
        sleep(Duration::from_secs(5));
        test_seg.record_end_time();

        assert_relative_eq!(test_seg.hours_total, 0.00139, epsilon = 1e-5f64);
    }
}