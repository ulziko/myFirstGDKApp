//! Bevy 3D frontend entry point.
//!
//! Step 3 expands this into the full 3D scene (sections 00-08 focus targets,
//! particle/grid backdrop, camera orbit transitions) plus the HTML/CSS
//! windowed HUD. This stub verifies that Bevy compiles for both the host and
//! `wasm32-unknown-unknown`.

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Minimal 3D stage placeholder — replaced by the monolith/particle scene in Step 3.
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 5.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1_500.0,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}
