use crate::assets::Assets;
use crate::game_difficulty::{GameDifficulty, GameDifficultyResource};
use crate::states::AppState;
use crate::ui_settings::*;
use bevy::prelude::*;

#[derive(Component)]
struct PlayButton {}

#[derive(Component)]
struct DifficultyButton {
    pressed: bool,
}

#[derive(Component)]
struct DifficultyText {}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), setup);
        app.add_systems(
            Update,
            update_difficulty_text.run_if(in_state(AppState::MainMenu)),
        );
        app.add_systems(
            Update,
            handle_play_button.run_if(in_state(AppState::MainMenu)),
        );
        app.add_systems(
            Update,
            handle_difficulty_button.run_if(in_state(AppState::MainMenu)),
        );
    }
}

fn setup(
    mut commands: Commands,
    assets: Res<Assets>,
    game_difficulty_resource: Res<GameDifficultyResource>,
) {
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
            DespawnOnExit(AppState::MainMenu),
        ))
        .with_children(|parent| {
            parent.spawn((
                Node {
                    margin: UiRect {
                        bottom: Val::Px(MD_MARGIN),
                        ..default()
                    },
                    ..default()
                },
                Text::new("Connect-4"),
                TextColor(Srgba::hex(PRIMARY_COLOR).unwrap().into()),
                TextFont {
                    font: assets.bold_font.clone(),
                    font_size: LG_FONT_SIZE,
                    ..default()
                },
            ));
            parent
                .spawn((
                    Node {
                        width: Val::Px(MD_BUTTON_WIDTH),
                        height: Val::Px(MD_BUTTON_HEIGHT),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        margin: UiRect {
                            bottom: Val::Px(SM_MARGIN),
                            ..default()
                        },
                        ..default()
                    },
                    BackgroundColor(Srgba::hex(PRIMARY_COLOR).unwrap().into()),
                    BorderRadius::all(Val::Px(BORDER_RADIUS)),
                    Button {},
                    PlayButton {},
                ))
                .with_children(|play_button| {
                    play_button.spawn((
                        Text::new("Play!"),
                        TextColor(Color::WHITE),
                        TextFont {
                            font: assets.font.clone(),
                            font_size: MD_FONT_SIZE,
                            ..default()
                        },
                    ));
                });

            parent
                .spawn((
                    Node {
                        width: Val::Px(MD_BUTTON_WIDTH),
                        height: Val::Px(MD_BUTTON_HEIGHT),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        margin: UiRect {
                            bottom: Val::Px(SM_MARGIN),
                            ..default()
                        },
                        ..default()
                    },
                    BackgroundColor(Srgba::hex(PRIMARY_COLOR).unwrap().into()),
                    BorderRadius::all(Val::Px(BORDER_RADIUS)),
                    Button {},
                    DifficultyButton { pressed: false },
                ))
                .with_children(|play_button| {
                    play_button.spawn((
                        DifficultyText {},
                        Text::new(get_difficulty_str(
                            &game_difficulty_resource.game_difficulty,
                        )),
                        TextColor(Color::WHITE),
                        TextFont {
                            font: assets.font.clone(),
                            font_size: MD_FONT_SIZE,
                            ..default()
                        },
                    ));
                });
        });
}

fn get_difficulty_str(game_difficulty: &GameDifficulty) -> &str {
    match game_difficulty {
        GameDifficulty::Easy => "Easy",
        GameDifficulty::Normal => "Normal",
        GameDifficulty::Hard => "Hard",
    }
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

fn next_difficulty(mut game_difficulty_resource: ResMut<GameDifficultyResource>) {
    match game_difficulty_resource.game_difficulty {
        GameDifficulty::Easy => {
            game_difficulty_resource.game_difficulty = GameDifficulty::Normal;
        }
        GameDifficulty::Normal => {
            game_difficulty_resource.game_difficulty = GameDifficulty::Hard;
        }
        GameDifficulty::Hard => {
            game_difficulty_resource.game_difficulty = GameDifficulty::Easy;
        }
    }
}

fn update_difficulty_text(
    game_difficulty_resource: Res<GameDifficultyResource>,
    mut query: Query<(&mut Text, &DifficultyText)>,
) {
    if !game_difficulty_resource.is_changed() {
        return;
    }

    if let Ok((mut text, _difficulty_text)) = query.single_mut() {
        *text = Text::new(get_difficulty_str(
            &game_difficulty_resource.game_difficulty,
        ));
    }
}

fn handle_difficulty_button(
    mut query: Query<(&Interaction, &mut DifficultyButton)>,
    game_difficulty_resource: ResMut<GameDifficultyResource>,
) {
    if let Ok((interaction, mut difficulty_button)) = query.single_mut() {
        match *interaction {
            Interaction::Pressed => {
                difficulty_button.pressed = true;
            }
            Interaction::Hovered => {
                if difficulty_button.pressed {
                    next_difficulty(game_difficulty_resource);
                    difficulty_button.pressed = false;
                }
            }
            Interaction::None => {
                difficulty_button.pressed = false;
            }
        }
    }
}
