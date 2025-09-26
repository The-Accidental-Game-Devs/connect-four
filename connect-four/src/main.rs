use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(1.0, 1.0, 1.0)))
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Connect Four".into(),
                name: Some("Connect Four".into()),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }),))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
