use crate::prelude::*;

pub fn spawn_tag(ecs: &mut World, name: &str) {
    ecs.spawn()
        .insert(Tag {
            name: name.to_string(),
            time_segments: vec![],
            is_active_segment: false,
            total_time: 0.0
        });
}