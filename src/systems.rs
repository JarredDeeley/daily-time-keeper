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
