#![warn(clippy::all, clippy::pedantic)]

use eframe::egui::{CentralPanel, CtxRef, ScrollArea, Button, TopBottomPanel, TextEdit, Key};
use eframe::epi::{App, Frame};
use eframe::{NativeOptions, run_native};
use time::{OffsetDateTime, Time};

mod tag;
mod time_segment;

use tag::*;
use time_segment::*;

pub fn main() {
    let app = TimeManager::new();
    let window_options = NativeOptions::default();
    run_native(Box::new(app), window_options);
}

struct TimeManager {
    tags: Vec<Tag>,
    tag_name: String,
    minute_rounding_scale: f32,
    minute_rounding_scale_field: String,
    is_rounding_on: bool,
}

impl TimeManager {
    fn new() -> TimeManager {
        TimeManager {
            tags: Vec::new(),
            tag_name: "".to_owned(),
            minute_rounding_scale: 0.25,
            minute_rounding_scale_field: "0.25".to_owned(),
            is_rounding_on: true,
        }
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
            });

            ui.horizontal(|ui| {
                // Rounding feature
                ui.label("Minute Rounding Scale: ");
                let minute_rounding_scale_response = ui.text_edit_singleline(&mut self.minute_rounding_scale_field);

                if minute_rounding_scale_response.lost_focus() {
                    match self.minute_rounding_scale_field.parse::<f32>() {
                        Ok(user_rounding_scale) => {
                            if self.minute_rounding_scale != user_rounding_scale {
                                self.minute_rounding_scale = user_rounding_scale
                            }
                        },
                        Err(_) => {
                            self.minute_rounding_scale_field = self.minute_rounding_scale.to_string()
                        },
                    }
                }

                ui.checkbox(&mut self.is_rounding_on, "Minute Rounding");
            });
        });

        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {

                let is_rounding_on = self.is_rounding_on;
                let minute_rounding_scale = self.minute_rounding_scale;

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
                                tag.end_time_segment(is_rounding_on, minute_rounding_scale);
                                tag.calculate_total();
                            } else {
                                tag.start_time_segment(is_rounding_on, minute_rounding_scale);
                            }
                        }

                        ui.separator();

                        ui.label(&tag.name);
                        ui.separator();
                        ui.label(format!("Total Hours: {}", &tag.total_time.to_string()));
                        ui.separator();

                        if ui.add(Button::new("Remove Tag")).clicked() {
                            tags_to_be_deleted.push(tag_index as u16);
                        }
                    });

                    ui.vertical(|ui| {
                        for segment in tag.time_segments.iter_mut() {
                            ui.horizontal(|ui| {
                                ui.add_space(40f32);
                                let start_hour_text = TextEdit::singleline(&mut segment.start_time_hour_field)
                                    .desired_width(25.);
                                let start_time_hour_field_response = ui.add(start_hour_text);

                                if start_time_hour_field_response.lost_focus() {
                                    match segment.start_time_hour_field.parse::<u8>() {
                                        Ok(user_hour) => {
                                            if user_hour < 24 {
                                                let old_time_hms = segment.start_time.unwrap().to_hms();
                                                let updated_time = segment.start_time.unwrap().replace_time(Time::from_hms(user_hour, old_time_hms.1, old_time_hms.2).unwrap());
                                                segment.start_time = Some(updated_time);
                                            } else {
                                                segment.start_time_hour_field = segment.start_time.unwrap().to_hms().0.to_string();
                                            }
                                        },
                                        Err(_) => {
                                            segment.start_time_hour_field = segment.start_time.unwrap().to_hms().0.to_string();
                                        },
                                    }

                                    if segment.end_time.is_some() {
                                        segment.calculate_total_hours();
                                    }
                                }

                                ui.label(":");

                                let start_minute_text = TextEdit::singleline(&mut segment.start_time_minute_field)
                                    .desired_width(25.);
                                let start_time_minute_field_response = ui.add(start_minute_text);

                                if start_time_minute_field_response.lost_focus() {
                                    match segment.start_time_minute_field.parse::<u8>() {
                                        Ok(user_minute) => {
                                            if user_minute < 60 {
                                                let old_time_hms = segment.start_time.unwrap().to_hms();
                                                let updated_time = segment.start_time.unwrap().replace_time(Time::from_hms(old_time_hms.0, user_minute, old_time_hms.2).unwrap());
                                                segment.start_time = Some(updated_time);
                                            } else {
                                                segment.start_time_minute_field = segment.start_time.unwrap().to_hms().1.to_string();
                                            }
                                        },
                                        Err(_) => {
                                            segment.start_time_minute_field = segment.start_time.unwrap().to_hms().1.to_string();
                                        },
                                    }

                                    if segment.end_time.is_some() {
                                        segment.calculate_total_hours();
                                    }
                                }

                                ui.add_space(20.);
                                ui.label("-");
                                ui.add_space(20.);

                                if segment.end_time.is_some() {
                                    let end_hour_text = TextEdit::singleline(&mut segment.end_time_hour_field)
                                        .desired_width(25.);
                                    let end_time_hour_response = ui.add(end_hour_text);
                                    if end_time_hour_response.lost_focus() {
                                        match segment.end_time_hour_field.parse::<u8>() {
                                            Ok(user_hour) => {
                                                if user_hour < 24 {
                                                    let old_time_hms = segment.end_time.unwrap().to_hms();
                                                    let updated_time = segment.end_time.unwrap().replace_time(Time::from_hms(user_hour, old_time_hms.1, old_time_hms.2).unwrap());
                                                    segment.end_time = Some(updated_time);
                                                } else {
                                                    segment.end_time_hour_field = segment.end_time.unwrap().to_hms().0.to_string();
                                                }
                                            },
                                            Err(_) => {
                                                segment.end_time_hour_field = segment.end_time.unwrap().to_hms().0.to_string();
                                            },
                                        }

                                        segment.calculate_total_hours();
                                    }

                                    ui.label(":");

                                    let end_minute_text = TextEdit::singleline(&mut segment.end_time_minute_field)
                                        .desired_width(25.);
                                    let end_time_minute_response = ui.add(end_minute_text);
                                    if end_time_minute_response.lost_focus() {
                                        match segment.end_time_minute_field.parse::<u8>() {
                                            Ok(user_minute) => {
                                                if user_minute < 60 {
                                                    let old_time_hms = segment.end_time.unwrap().to_hms();
                                                    let updated_time = segment.end_time.unwrap().replace_time(Time::from_hms(old_time_hms.0, user_minute, old_time_hms.2).unwrap());
                                                    segment.end_time = Some(updated_time);
                                                } else {
                                                    segment.end_time_minute_field = segment.end_time.unwrap().to_hms().1.to_string();
                                                }
                                            },
                                            Err(_) => {
                                                segment.end_time_minute_field = segment.end_time.unwrap().to_hms().1.to_string();
                                            },
                                        }

                                        segment.calculate_total_hours();
                                    }
                                }
                                ui.separator();
                                ui.label(format!("Hours: {:.2}", segment.hours_total))
                            });
                        }
                        tag.calculate_total();

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

#[cfg(test)]
mod tests {
    use std::thread::sleep;
    use std::time::Duration;
    use super::*;

    #[test]
    fn test_start_segment() {
        let mut test_tag = Tag::new("test");
        test_tag.start_time_segment(false, 0.0);

        assert_eq!(test_tag.time_segments.len(), 1)
    }

    #[test]
    fn test_end_segment() {
        let mut test_tag = Tag::new("test");
        test_tag.start_time_segment(false, 0.0);
        test_tag.end_time_segment(false, 0.0);

        assert_eq!(test_tag.is_active_segment, false);
    }

    #[test]
    fn test_calculate_total_hours() {
        let mut test_tag = Tag::new("test");
        test_tag.start_time_segment(false, 0.0);
        sleep(Duration::from_secs(5));
        test_tag.end_time_segment(false, 0.0);

        assert_eq!(test_tag.time_segments.last().unwrap().hours_total, 0.001388888888888889);
    }

    #[test]
    fn test_time_rounding() {
        let time_manager = TimeSegment::new(false, 0.0);
        let rounding_scale = 0.25;
        let times = [
            [Time::from_hms(6,25, 0), Time::from_hms(6,30, 0)],
            [Time::from_hms(0,1, 0), Time::from_hms(0,0, 0)],
            [Time::from_hms(0,7, 0), Time::from_hms(0,0, 0)],
            [Time::from_hms(0,8, 0), Time::from_hms(0,15, 0)],
            [Time::from_hms(0,14, 0), Time::from_hms(0,15, 0)],
            [Time::from_hms(0,15, 0), Time::from_hms(0,15, 0)],
            [Time::from_hms(0,16, 0), Time::from_hms(0,15, 0)],
            [Time::from_hms(0,53, 0), Time::from_hms(1,0, 0)],
            [Time::from_hms(0,59, 0), Time::from_hms(1,0, 0)],
            [Time::from_hms(22, 58, 0), Time::from_hms(23, 0, 0)],
            [Time::from_hms(11, 58, 0), Time::from_hms(12, 0, 0)]];

        for time in times.iter() {
            let unrounded_time = time[0].unwrap().as_hms();
            let rounded_time = time_manager.round_time((unrounded_time.0, unrounded_time.1), rounding_scale);
            assert_eq!(Time::from_hms(rounded_time.0, rounded_time.1, 0), time[1]);
        }
    }
}
