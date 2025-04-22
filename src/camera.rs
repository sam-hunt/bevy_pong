use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    // Create a persistent camera that will be used throughout the game
    commands.spawn((
        Camera2d,
        Msaa::Off,
        // Mark this as the main camera
        MainCamera,
    ));
}

#[derive(Component)]
pub struct MainCamera;
