use crate::prelude::*;

#[derive(Component)]
pub struct Tag {
    pub name: String,
    pub time_segments: Vec<TimeSegment>,
    pub is_active_segment: bool,
    pub total_time: f64,
}