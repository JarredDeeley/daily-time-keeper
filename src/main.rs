#![warn(clippy::all, clippy::pedantic)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod components;
mod systems;
mod spawner;
mod events;

mod prelude {
    pub use std::fs::File;
    pub use std::io::{Read, Write};
    pub use std::path::Path;
    pub use time::{Time, OffsetDateTime};
    pub use serde::{Serialize, Deserialize};
    pub use bevy::prelude::*;
    pub use bevy_egui::{
        egui,
        EguiContext,
        EguiPlugin,
    };

    pub use crate::components::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::events::*;
}

use prelude::*;

const APP_NAME : &str = "Daily Time Keeper";

pub fn main() {
    App::new()
    .init_resource::<TimeManagerState>()
    .add_plugins(DefaultPlugins)
    .add_plugin(EguiPlugin)
    .add_event::<WantsToCreateTag>()
    .add_event::<WantsToCreateTimeSegment>()
    .add_event::<WantsToFlipActiveState>()
    .add_startup_system(state_setup)
    .add_system(tag_creation_reader)
    .add_system(time_segment_reader)
    .add_system(flip_active_state_reader)
    .add_system(time_manager)
    .add_system(end_active_time_segment)
    .add_system(calculate_time_segment)
    .add_system(calculate_tag_time_total)
    .add_system(remove_components
        .after(calculate_time_segment)
        .after(calculate_tag_time_total)
    )
    .run();
}

fn state_setup(mut state: ResMut<TimeManagerState>) {
    state.tag_name = "".to_owned();
    state.minute_rounding_scale = 0.25;
    state.minute_rounding_scale_field = "0.25".to_owned();
    state.is_rounding_on = true;
    state.is_dark_mode = true;
}

fn time_manager(
    mut commands: Commands,
    mut egui_context: ResMut<EguiContext>,
    mut tag_creation_writer: EventWriter<WantsToCreateTag>,
    mut time_segment_creation_writer: EventWriter<WantsToCreateTimeSegment>,
    mut flip_active_state_writer: EventWriter<WantsToFlipActiveState>,
    mut state: ResMut<TimeManagerState>,
    tag_query: Query<(Entity, &Tag)>,
    mut time_segment_query: Query<(Entity, &Parent, &mut TimeSegment, Option<&ActiveTimeSegment>)>,
) {
    if state.is_dark_mode {
        egui_context.ctx_mut().set_visuals(egui::Visuals::dark());
    } else {
        egui_context.ctx_mut().set_visuals(egui::Visuals::light());
    }

    egui::TopBottomPanel::top("Panel").show(egui_context.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            // Adding new tag
            let new_tag_response = ui.add(egui::TextEdit::singleline(&mut state.tag_name).hint_text("Enter Tag Name"));
            if new_tag_response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                if !state.tag_name.is_empty() {
                    tag_creation_writer.send(WantsToCreateTag{
                        name: state.tag_name.clone(),
                    });
                    state.tag_name = "".to_string();
                }
            }
            if ui.add(egui::Button::new("Add New Tag")).clicked() {
                if !state.tag_name.is_empty() {
                    tag_creation_writer.send(WantsToCreateTag{
                        name: state.tag_name.clone(),
                    });
                    state.tag_name = "".to_string();
                }
            }
        });

        ui.horizontal(|ui| {
            // Rounding feature
            ui.label("Minute Rounding Scale: ");
            let minute_rounding_scale_response = ui.text_edit_singleline(&mut state.minute_rounding_scale_field);

            if minute_rounding_scale_response.lost_focus() {
                match state.minute_rounding_scale_field.parse::<f32>() {
                    Ok(user_rounding_scale) => {
                        if state.minute_rounding_scale != user_rounding_scale {
                            state.minute_rounding_scale = user_rounding_scale;
                        }
                    },
                    Err(_) => {
                        state.minute_rounding_scale_field = state.minute_rounding_scale.to_string();
                    },
                }
            }

            ui.checkbox(&mut state.is_rounding_on, "Minute Rounding");
            ui.separator();
            ui.checkbox(&mut state.is_dark_mode, "Dark Mode");
        });
    });

    egui::CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            for (id, tag) in tag_query.iter() {
                let mut end_time_segment = false;

                ui.horizontal(|ui| {
                    let button_text;
                    if tag.is_active_segment {
                        button_text = "Stop";
                    } else {
                        button_text = "Start";
                    }

                    if ui.add(egui::Button::new(button_text)).clicked() {
                        if tag.is_active_segment {
                            end_time_segment = true;
                        } else {
                            time_segment_creation_writer.send(WantsToCreateTimeSegment {
                                tag: id,
                            });
                        }

                        flip_active_state_writer.send(WantsToFlipActiveState {
                            tag_id: id,
                        });
                    }

                    ui.separator();

                    ui.label(&tag.name);
                    ui.separator();
                    ui.label(format!("Total Hours: {}", &tag.total_time.to_string()));

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                        if ui.add(egui::Button::new("Remove Tag")).clicked() {
                            commands
                                .entity(id)
                                .insert(RemoveMe);
                        }
                    });

                });

                // time segment query
                for (segment_entity, parent, mut segment, active_segment) in time_segment_query.iter_mut() {
                    if parent.get() == id {
                        if let Some(_active_segment) = active_segment {
                            if end_time_segment {
                                commands
                                    .entity(segment_entity)
                                    .insert(EndTimeSegment);
                            }
                        }


                        ui.vertical(|ui| {
                            ui.horizontal(|ui| {
                                ui.add_space(40f32);
                                let start_hour_text = egui::TextEdit::singleline(&mut segment.start_time_hour_field)
                                    .desired_width(25.);
                                let start_time_hour_field_response = ui.add(start_hour_text);

                                ui.label(":");

                                let start_minute_text = egui::TextEdit::singleline(&mut segment.start_time_minute_field)
                                    .desired_width(25.);
                                let start_time_minute_field_response = ui.add(start_minute_text);

                                ui.add_space(20.);
                                ui.label("-");
                                ui.add_space(20.);


                                if segment.end_time.is_some() {
                                    let end_hour_text = egui::TextEdit::singleline(&mut segment.end_time_hour_field)
                                        .desired_width(25.);
                                    let end_time_hour_response = ui.add(end_hour_text);

                                    ui.label(":");

                                    let end_minute_text = egui::TextEdit::singleline(&mut segment.end_time_minute_field)
                                        .desired_width(25.);
                                    let end_time_minute_response = ui.add(end_minute_text);

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
                                    }

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
                                    }
                                }

                                ui.separator();
                                ui.label(format!("Hours: {:.2}", segment.hours_total));
                                ui.add_space(20.);

                                ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                                    if ui.add(egui::Button::new("Remove Time Segment")).clicked() {
                                        commands
                                            .entity(segment_entity)
                                            .insert(RemoveMe);
                                    }
                                });

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
                                }

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
                                }
                            });
                        });
                    }
                }
            }

            ui.add_space(20.);
        });
    });

    egui::TopBottomPanel::bottom("bottom_panel").show(egui_context.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            if ui.add(egui::Button::new("Clear Session")).clicked() {
                for (segment_entity, _parent, _segment, _active_segment) in time_segment_query.iter_mut() {
                    commands
                        .entity(segment_entity)
                        .insert(RemoveMe);
                }
            }
            // ui.separator();
            // ui.label("End session & save");
        });
    });
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
