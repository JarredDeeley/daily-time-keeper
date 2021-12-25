#![warn(clippy::all, clippy::pedantic)]
use iced::{Element, Sandbox, Settings, Text};

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

    fn start_time(&mut self) {
        if self.is_active_segment == false {
            let new_segment = TimeSegment;
            self.time_segments.push(new_segment);
            self.is_active_segment = true;
        } else {
            println!("Active segment already exists");
        }
    }

    fn end_time(&mut self) {
    }

    fn calculate_total(&mut self) {
    }
}

#[derive(PartialEq)]
struct TimeSegment;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_segment() {
        let mut test_tag = TagGroup::new("test");
        test_tag.start_time();

        assert_eq!(test_tag.time_segments.len(), 1)
    }

    #[test]
    fn test_end_segment() {
        let mut test_tag = TagGroup::new("test");
        test_tag.start_time();
        test_tag.end_time();

        assert_eq!(test_tag.is_active_segment, false);
    }
}