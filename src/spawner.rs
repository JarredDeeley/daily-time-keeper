use crate::prelude::*;

pub fn spawn_tag(mut commands: Commands, name: String) {
    commands
        .spawn()
        .insert(Tag {
            name,
            is_active_segment: false,
            total_time: 0.0
        });
}

pub fn spawn_time_segment(ecs: &mut World, tag: &Entity) {
    ecs.spawn()
        .insert(TimeSegment {
            start_time: None,
            end_time: None,
            start_time_hour_field: "".to_string(),
            start_time_minute_field: "".to_string(),
            end_time_hour_field: "".to_string(),
            end_time_minute_field: "".to_string(),
            hours_total: 0.0
        });
}