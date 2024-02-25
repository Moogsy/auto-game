use bevy::prelude::*;

use crate::capabilities::CanTranslate;

#[derive(Bundle)]
pub struct Blob {
    sprite_bundle: SpriteBundle,
    can_translate: CanTranslate
}
