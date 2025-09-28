use crate::states::AppState;
use bevy::prelude::*;

#[derive(Component)]
struct MainMenu {}

#[derive(Component)]
struct PlayButton {}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), setup);
        app.add_systems(
            Update,
            handle_play_button.run_if(in_state(AppState::MainMenu)),
        );
        app.add_systems(OnExit(AppState::MainMenu), unload_main_menu);
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            MainMenu {},
        ))
        .with_children(|parent| {
            parent.spawn((
                Node {
                    margin: UiRect {
                        bottom: Val::Px(24.0),
                        ..default()
                    },
                    ..default()
                },
                Text::new("Connect4"),
                TextColor(Color::BLACK),
                TextFont {
                    font_size: 64.0,
                    ..default()
                },
            ));
            parent
                .spawn((
                    Node {
                        width: Val::Px(250.0),
                        height: Val::Px(60.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    BackgroundColor(Color::BLACK),
                    BorderRadius::all(Val::Px(24.0)),
                    Button {},
                    PlayButton {},
                ))
                .with_children(|play_button| {
                    play_button.spawn(Text::new("Play!"));
                });
        });
}

fn handle_play_button(
    query: Query<(&Interaction, &PlayButton)>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, _play_button)) = query.single() {
        match *interaction {
            Interaction::Pressed => {
                next_state.set(AppState::InGame);
            }
            _ => {}
        }
    }
}

fn unload_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu) = query.single() {
        commands.entity(main_menu).despawn();
    }
}
