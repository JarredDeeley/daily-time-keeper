use crate::prelude::*;

#[derive(Component)]
pub struct Tag {
    pub name: String,
    pub is_active_segment: bool,
    pub total_time: f64,
}

#[derive(Component)]
pub struct TimeSegment {
    pub start_time: Option<OffsetDateTime>,
    pub end_time: Option<OffsetDateTime>,
    pub start_time_hour_field: String,
    pub start_time_minute_field: String,
    pub end_time_hour_field: String,
    pub end_time_minute_field: String,
    pub hours_total: f64,
}

#[derive(Default)]
pub struct TimeManagerState {
    pub tag_name: String,
    pub minute_rounding_scale: f32,
    pub minute_rounding_scale_field: String,
    pub is_rounding_on: bool,
    pub is_dark_mode: bool,
}

#[derive(Component)]
pub struct ActiveTimeSegment;

#[derive(Component)]
pub struct EndTimeSegment;

#[derive(Component)]
pub struct RemoveMe;
