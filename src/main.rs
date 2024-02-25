use bevy::prelude::*;

mod blobs;
mod camera;
mod capabilities;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(capabilities::CapabilitiesPlugin)
        .run();
}
