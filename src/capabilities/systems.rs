use bevy::prelude::*;

use super::{CanTranslate, Translating};

pub fn translate_entities(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &CanTranslate), With<Translating>>,
) {
    query
        .par_iter_mut()
        .for_each(|(mut transform, can_translate)| {
            let translation_direction = transform.rotation * Vec3::Y;
            let movement_distance = can_translate.get_speed() * time.delta_seconds();
            let translation_delta = movement_distance * translation_direction;
            transform.translation += translation_delta;
        }
    );
}
