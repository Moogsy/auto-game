use bevy::prelude::*;

/// Marks an entity as one able to translate in any direction
#[derive(Component)]
pub struct CanTranslate {
    // The speed at which the entity can translate
    // Must be positive
    speed: f32
}

impl CanTranslate {
    /// Instantiate this struct 
    pub fn new(speed: f32) -> Option<Self> {
        if speed >= 0.0 {
            Some(Self { speed })
        } else {
            None
        }
    }
    /// Gets the speed at which the entity this component is attached to can move
    pub fn get_speed(&self) -> f32 {
        self.speed
    }
}

/// Marks an entity as one that is currently translating
/// Must be put on an entity that already has the CanTranslate component
#[derive(Component)]
pub struct Translating;

