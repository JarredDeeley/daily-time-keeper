use crate::prelude::*;

pub struct WantsToCreateTag {
    pub name: String,
}

pub fn tag_creation_reader(mut commands: Commands, mut events: EventReader<WantsToCreateTag>) {
    for event in events.iter() {
        commands
            .spawn()
            .insert(Tag {
                name: event.name.clone(),
                is_active_segment: false,
                total_time: 0.0
            });
    }
}

pub struct WantsToCreateTimeSegment {
    pub tag: Entity,
}

pub fn time_segment_reader(
    mut commands: Commands,
    mut events: EventReader<WantsToCreateTimeSegment>,
    state: ResMut<TimeManagerState>,
    tag_query: Query<(Entity, &Tag)>,
) {
    let mut current_time = OffsetDateTime::now_local().ok();

    if state.is_rounding_on {
        let current_time_hms = current_time.unwrap().to_hms();
        let rounded_time = round_time(
            (current_time_hms.0, current_time_hms.1), state.minute_rounding_scale);
        let offset_rounded_time = current_time.unwrap().replace_time(Time::from_hms(rounded_time.0, rounded_time.1, 0).unwrap());
        current_time = Some(offset_rounded_time);
    }
    let start_time = current_time;
    let start_time_formatted = format_time_hour_minute(start_time.unwrap());
    let start_time_hour_field = start_time_formatted.0;
    let start_time_minute_field = start_time_formatted.1;

    for event in events.iter() {
        let segment = commands
            .spawn()
            .insert(TimeSegment {
                start_time,
                end_time: None,
                start_time_hour_field: start_time_hour_field.to_owned(),
                start_time_minute_field: start_time_minute_field.to_owned(),
                end_time_hour_field: "".to_string(),
                end_time_minute_field: "".to_string(),
                hours_total: 0.0
            })
            .insert(ActiveTimeSegment)
            .id();

        tag_query.for_each(|(entity, _tag)| {
           if entity == event.tag {
               commands.entity(entity).add_child(segment);
           }
        });
        // commands.entity(tag_query.get(event.tag)).add_child(segment);
    }
}

pub struct WantsToFlipActiveState {
    pub tag_id: Entity,
}

pub fn flip_active_state_reader(
    mut events: EventReader<WantsToFlipActiveState>,
    mut tag_query: Query<(Entity, &mut Tag)>,
) {
    for event in events.iter() {
        for (id, mut tag) in tag_query.iter_mut() {
            if id == event.tag_id {
                tag.is_active_segment = !tag.is_active_segment;
            }
        }
    }
}
