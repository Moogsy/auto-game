use bevy::prelude::*;

mod components;
pub use components::*;

mod systems;

pub struct CapabilitiesPlugin;

impl Plugin for CapabilitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, systems::translate_entities);
    }
}
