#![warn(clippy::all, clippy::pedantic)]

use std::iter::FromIterator;
use eframe::egui::{CentralPanel, CtxRef, ScrollArea, Button};
use eframe::epi::{App, Frame};
use eframe::{NativeOptions, run_native};
use time::OffsetDateTime;

pub fn main() {
    let app = TimeManager::new();
    let window_options = NativeOptions::default();
    run_native(Box::new(app), window_options);
}

struct TimeManager {
    tags: Vec<Tag>,
}

impl TimeManager {
    fn new() -> TimeManager {
        let dummy_iter = (0..3).map(|dummy|
            Tag::new(format!("Tag:{}", dummy))
        );

        TimeManager {
            tags: Vec::from_iter(dummy_iter)
        }
    }
}

impl App for TimeManager {
    fn update(&mut self, ctx: &CtxRef, frame: &Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Time Tags");
            ScrollArea::vertical().show(ui, |ui| {
                for tag in self.tags.iter_mut() {
                    ui.horizontal(|ui| {
                        let button_text;
                        if tag.is_active_segment {
                            button_text = "Stop";
                        } else {
                            button_text = "Start";
                        }

                        if ui.add(Button::new(button_text)).clicked() {
                            if tag.is_active_segment {
                                tag.end_time_segment();
                                tag.calculate_total();
                            } else {
                                tag.start_time_segment();
                            }
                        }
                        ui.label(&tag.name);
                        ui.label(format!("Total Hours: {}", &tag.total_time.to_string()));
                    });

                    ui.vertical(|ui| {
                        for segment in tag.time_segments.iter_mut() {
                            ui.horizontal(|ui| {
                                ui.label(format_time_hms(segment.start_time));
                                ui.label(" - ");
                                if segment.end_time.is_some() {
                                    ui.label(format_time_hms(segment.end_time.unwrap()));
                                }
                                ui.separator();
                                ui.label(format!("Hours: {}", segment.hours_total))
                            });
                        }

                        ui.separator();
                    });
                }
            });
        });
    }

    fn name(&self) -> &str {
        "Daily Time Keeper"
    }
}

fn format_time_hms(time_stamp: OffsetDateTime) -> String {
    format!("{}:{}:{}", time_stamp.to_hms().0, time_stamp.to_hms().1, time_stamp.to_hms().2)
}

struct Tag {
    name: String,
    time_segments: Vec<TimeSegment>,
    is_active_segment: bool,
    total_time: f64,
}

impl Tag {
    fn new(name: String) -> Tag {
        let mut _self = Tag {
            name,
            time_segments: Vec::new(),
            is_active_segment: false,
            total_time: 0f64,
        };

        _self
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

    fn end_time_segment(&mut self) {
        self.is_active_segment = false;
        self.time_segments
            .last_mut()
            .unwrap()
            .record_end_time();
    }

    fn calculate_total(&mut self) {
        let mut running_time = 0f64;

        for time_segment in self.time_segments.iter() {
            running_time += time_segment.hours_total;
        }

        self.total_time = running_time;
    }
}

struct TimeSegment {
    start_time: OffsetDateTime,
    end_time: Option<OffsetDateTime>,
    hours_total: f64,
}

impl TimeSegment {
    fn new() -> TimeSegment {
        let mut _self = TimeSegment {
            start_time: OffsetDateTime::now_local().unwrap(),
            end_time: None,
            hours_total: 0f64,
        };

        _self
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
    use super::*;

    #[test]
    fn test_start_segment() {
        let mut test_tag = Tag::new("test".to_string());
        test_tag.start_time_segment();

        assert_eq!(test_tag.time_segments.len(), 1)
    }

    #[test]
    fn test_end_segment() {
        let mut test_tag = Tag::new("test".to_string());
        test_tag.start_time_segment();
        test_tag.end_time_segment();

        assert_eq!(test_tag.is_active_segment, false);
    }

    #[test]
    fn test_calculate_total_hours() {
        let mut test_tag = Tag::new("test".to_string());
        test_tag.start_time_segment();
        sleep(Duration::from_secs(5));
        test_tag.end_time_segment();

        assert_eq!(test_tag.time_segments.last().unwrap().hours_total, 0.001388888888888889);
    }
}
