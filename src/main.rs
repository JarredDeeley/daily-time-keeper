#![warn(clippy::all, clippy::pedantic)]

use std::cmp::min;
use eframe::egui::{CentralPanel, CtxRef, ScrollArea, Button, TopBottomPanel, TextEdit, Key};
use eframe::epi::{App, Frame};
use eframe::{NativeOptions, run_native};
use time::{OffsetDateTime, Time};

pub fn main() {
    let app = TimeManager::new();
    let window_options = NativeOptions::default();
    run_native(Box::new(app), window_options);
}

struct TimeManager {
    tags: Vec<Tag>,
    tag_name: String,
    minute_rounding_scale: f32,
    is_rounding_on: bool,
}

impl TimeManager {
    fn new() -> TimeManager {
        TimeManager {
            tags: Vec::new(),
            tag_name: "".to_owned(),
            minute_rounding_scale: 0.25,
            is_rounding_on: true,
        }
    }

    fn round_time(&self, time_to_be_rounded: (u8, u8)) -> (u8, u8) {
        let minute_accuracy = (60.0 * self.minute_rounding_scale).floor();
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

impl App for TimeManager {
    fn update(&mut self, ctx: &CtxRef, frame: &Frame) {
        let mut tags_to_be_deleted: Vec<u16> = Vec::new();

        TopBottomPanel::top("Panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                // Adding new tag
                let new_tag_response = ui.add(TextEdit::singleline( &mut self.tag_name).hint_text("Enter Tag Name"));
                if new_tag_response.lost_focus() && ui.input().key_pressed(Key::Enter) {
                    if !self.tag_name.is_empty() {
                        self.tags.push(Tag::new(self.tag_name.as_str()));
                        self.tag_name = "".to_string();
                    }
                }
                if ui.add(Button::new("Add New Tag")).clicked() {
                    if !self.tag_name.is_empty() {
                        self.tags.push(Tag::new(self.tag_name.as_str()));
                        self.tag_name = "".to_string();
                    }
                }

                // Rounding feature
                ui.separator();
                ui.label(format!("Minute Rounding Scale: [ {} ]", self.minute_rounding_scale));
                if self.is_rounding_on {
                    ui.label("rounding enabled");
                } else {
                    ui.label("rounding disabled");
                }

            });
        });

        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                for (tag_index, tag) in self.tags.iter_mut().enumerate() {
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

                        if ui.add(Button::new("Remove Tag")).clicked() {
                            tags_to_be_deleted.push(tag_index as u16);
                        }
                    });

                    ui.vertical(|ui| {
                        for segment in tag.time_segments.iter_mut() {
                            ui.horizontal(|ui| {
                                ui.add_space(40f32);
                                ui.label(format_time_hms(segment.start_time));
                                ui.label(" - ");
                                if segment.end_time.is_some() {
                                    ui.label(format_time_hms(segment.end_time.unwrap()));
                                }
                                ui.separator();
                                ui.label(format!("Hours: {:.2}", segment.hours_total))
                            });
                        }

                        ui.separator();
                    });
                }

                for tag_index in tags_to_be_deleted.drain(..) {
                    self.tags.remove(tag_index as usize);
                }
            });
        });

        TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.add(Button::new("Clear Session")).clicked() {
                    for tag in self.tags.iter_mut() {
                        tag.clear_session();
                    }
                }
                ui.separator();
                ui.label("End session & save");
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
    fn new(name: &str) -> Tag {
        let name = name.to_string();
        let mut _self = Tag {
            name,
            time_segments: Vec::new(),
            is_active_segment: false,
            total_time: 0f64,
        };

        _self
    }

    fn clear_session(&mut self) {
        self.time_segments.clear();
        self.is_active_segment = false;
        self.total_time = 0f64;
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
        let mut test_tag = Tag::new("test");
        test_tag.start_time_segment();

        assert_eq!(test_tag.time_segments.len(), 1)
    }

    #[test]
    fn test_end_segment() {
        let mut test_tag = Tag::new("test");
        test_tag.start_time_segment();
        test_tag.end_time_segment();

        assert_eq!(test_tag.is_active_segment, false);
    }

    #[test]
    fn test_calculate_total_hours() {
        let mut test_tag = Tag::new("test");
        test_tag.start_time_segment();
        sleep(Duration::from_secs(5));
        test_tag.end_time_segment();

        assert_eq!(test_tag.time_segments.last().unwrap().hours_total, 0.001388888888888889);
    }

    #[test]
    fn test_time_rounding() {
        let time_manager = TimeManager::new();
        let times = [
            [Time::from_hms(6,25, 0), Time::from_hms(6,30, 0)],
            [Time::from_hms(0,1, 0), Time::from_hms(0,0, 0)],
            [Time::from_hms(0,7, 0), Time::from_hms(0,0, 0)],
            [Time::from_hms(0,8, 0), Time::from_hms(0,15, 0)],
            [Time::from_hms(0,14, 0), Time::from_hms(0,15, 0)],
            [Time::from_hms(0,15, 0), Time::from_hms(0,15, 0)],
            [Time::from_hms(0,16, 0), Time::from_hms(0,15, 0)],
            [Time::from_hms(0,53, 0), Time::from_hms(1,0, 0)],
            [Time::from_hms(0,59, 0), Time::from_hms(1,0, 0)]];

        for time in times.iter() {
            let unrounded_time = time[0].unwrap().as_hms();
            let rounded_time = time_manager.round_time((unrounded_time.0, unrounded_time.1));
            assert_eq!(Time::from_hms(rounded_time.0, rounded_time.1, 0), time[1]);
        }
    }
}
