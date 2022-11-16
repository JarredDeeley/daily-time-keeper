use crate::prelude::*;


pub fn remove_components(
    mut commands: Commands,
    components_selected_for_removal: Query<(Entity, &RemoveMe)>,
) {
    for (entity, _remove_comp) in components_selected_for_removal.iter() {
        commands
            .entity(entity)
            .despawn_recursive();
    }
}

pub fn end_active_time_segment(
    mut commands: Commands,
    state: ResMut<TimeManagerState>,
    mut segment_query: Query<(Entity, &mut TimeSegment), (With<EndTimeSegment>, With<ActiveTimeSegment>)>,
) {
    segment_query.for_each_mut(|(entity, mut segment)| {
        // generate end time
        let mut current_time = OffsetDateTime::now_local().ok();

        if state.is_rounding_on {
            let current_time_hms = current_time.unwrap().to_hms();
            let rounded_time = round_time(
                (current_time_hms.0, current_time_hms.1), state.minute_rounding_scale);
            let offset_rounded_time = current_time.unwrap().replace_time(Time::from_hms(rounded_time.0, rounded_time.1, 0).unwrap());
            current_time = Some(offset_rounded_time);
        }

        segment.end_time = current_time;
        let end_time_formatted = format_time_hour_minute(segment.end_time.unwrap());
        segment.end_time_hour_field = end_time_formatted.0;
        segment.end_time_minute_field = end_time_formatted.1;

        // unmark the segment as active
        commands
            .entity(entity)
            .remove::<ActiveTimeSegment>()
            .remove::<EndTimeSegment>();
    })
}

pub fn round_time(time_to_be_rounded: (u8, u8), minute_rounding_scale: f32) -> (u8, u8) {
    let minute_accuracy = (60.0 * minute_rounding_scale).floor();
    let mut rounded_time = time_to_be_rounded;

    let mut minutes = time_to_be_rounded.1;
    minutes = ((minutes as f32 / minute_accuracy + 0.5).floor() * minute_accuracy) as u8;

    if minutes >= 60 {
        rounded_time.0 += 1;
        if rounded_time.0 > 23 {
            rounded_time.0 = 0;
        }
        rounded_time.1 = minutes % 60;
    } else {
        rounded_time.1 = minutes;
    }

    rounded_time
}

pub fn format_time_hour_minute(time_stamp: OffsetDateTime) -> (String, String) {
    (time_stamp.to_hms().0.to_string(), time_stamp.to_hms().1.to_string())
}

pub fn calculate_time_segment(
    mut segment_query: Query<&mut TimeSegment>,
) {
    segment_query.for_each_mut(|mut segment| {
        if let Some(end_time) = segment.end_time {
            let time_duration = end_time - segment.start_time.unwrap();
            segment.hours_total = time_duration.as_seconds_f64() / 3600f64;
        }
    })
}

pub fn calculate_tag_time_total(
    mut tag_query: Query<(Entity, &mut Tag, &Children)>,
    segment_query: Query<&TimeSegment>,
) {
    for (_, mut tag, children) in tag_query.iter_mut() {
        let mut total_hours = 0f64;

        for &child in children.iter() {
            let segment = segment_query.get(child);

            total_hours += segment.unwrap().hours_total;
        }

        tag.total_time =  total_hours;
    }
}
