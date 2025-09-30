use crate::game_result::{GameResult, Result};
use crate::states::{AppState, GameState};
use crate::ui_settings::*;
use bevy::prelude::*;

#[derive(Component)]
struct GameOverUi {}

#[derive(Component)]
struct GameOverText {}

#[derive(Component)]
struct BackButton {}

#[derive(Component)]
struct ReplayButton {}

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup);
        app.add_systems(
            Update,
            handle_back_button.run_if(in_state(AppState::InGame)),
        );
        app.add_systems(
            Update,
            handle_replay_button
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(GameState::GameOver)),
        );
        app.add_systems(
            OnEnter(GameState::GameOver),
            update_game_over_text.run_if(in_state(AppState::InGame)),
        );
        app.add_systems(
            OnEnter(GameState::GameOver),
            show_game_over.run_if(in_state(AppState::InGame)),
        );
        app.add_systems(
            OnExit(GameState::GameOver),
            hide_game_over.run_if(in_state(AppState::InGame)),
        );
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Auto,
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Start,
                padding: UiRect::all(Val::Px(PADDING)),
                ..default()
            },
            StateScoped(AppState::InGame),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Px(SM_BUTTON_WIDTH),
                        height: Val::Px(SM_BUTTON_HEIGHT),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    BackgroundColor(Srgba::hex(PRIMARY_COLOR).unwrap().into()),
                    BorderRadius::all(Val::Px(BORDER_RADIUS)),
                    Button {},
                    BackButton {},
                ))
                .with_children(|back_button| {
                    back_button.spawn((
                        Text::new("Back"),
                        TextFont {
                            font_size: SM_FONT_SIZE,
                            ..default()
                        },
                    ));
                });
        });

    commands
        .spawn((
            GameOverUi {},
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            Visibility::Hidden,
            StateScoped(AppState::InGame),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Auto,
                        height: Val::Auto,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(PADDING)),
                        ..default()
                    },
                    BackgroundColor(Srgba::hex(WHITE_OVERLAY).unwrap().into()),
                    BorderRadius::all(Val::Px(BORDER_RADIUS)),
                ))
                .with_children(|game_over| {
                    game_over.spawn((
                        GameOverText {},
                        Node {
                            margin: UiRect::all(Val::Px(SM_MARGIN)),
                            ..default()
                        },
                        Text::new("Undefine"),
                        TextColor(Color::BLACK),
                        TextFont {
                            font_size: LG_FONT_SIZE,
                            ..default()
                        },
                    ));
                    game_over
                        .spawn((
                            Node {
                                width: Val::Px(MD_BUTTON_WIDTH),
                                height: Val::Px(MD_BUTTON_HEIGHT),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                margin: UiRect::all(Val::Px(SM_MARGIN)),
                                ..default()
                            },
                            BackgroundColor(Srgba::hex(PRIMARY_COLOR).unwrap().into()),
                            BorderRadius::all(Val::Px(BORDER_RADIUS)),
                            Button {},
                            ReplayButton {},
                        ))
                        .with_children(|back_button| {
                            back_button.spawn((
                                Text::new("Replay!"),
                                TextFont {
                                    font_size: MD_FONT_SIZE,
                                    ..default()
                                },
                            ));
                        });
                    game_over
                        .spawn((
                            Node {
                                width: Val::Px(MD_BUTTON_WIDTH),
                                height: Val::Px(MD_BUTTON_HEIGHT),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                margin: UiRect::all(Val::Px(SM_MARGIN)),
                                ..default()
                            },
                            BackgroundColor(Srgba::hex(PRIMARY_COLOR).unwrap().into()),
                            BorderRadius::all(Val::Px(BORDER_RADIUS)),
                            Button {},
                            BackButton {},
                        ))
                        .with_children(|back_button| {
                            back_button.spawn((
                                Text::new("Back"),
                                TextFont {
                                    font_size: MD_FONT_SIZE,
                                    ..default()
                                },
                            ));
                        });
                });
        });
}

fn update_game_over_text(
    mut query: Query<(&mut Text, &GameOverText)>,
    game_result: Res<GameResult>,
) {
    if let Ok((mut text, _game_over_text)) = query.single_mut() {
        let game_over_text = match game_result.result {
            Result::PlayerWon => "Player won!",
            Result::BotWon => "Bot won!",
            Result::Draw => "Draw",
            Result::Unknow => "Undefined",
        };
        *text = Text::new(game_over_text);
    }
}

fn show_game_over(mut query: Query<(&mut Visibility, &GameOverUi)>) {
    if let Ok((mut visibility, _game_over_ui)) = query.single_mut() {
        *visibility = Visibility::Visible;
    }
}

fn hide_game_over(mut query: Query<(&mut Visibility, &GameOverUi)>) {
    if let Ok((mut visibility, _game_over_ui)) = query.single_mut() {
        *visibility = Visibility::Hidden;
    }
}

fn handle_back_button(
    query: Query<(&Interaction, &BackButton)>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, _back_button) in query {
        match *interaction {
            Interaction::Pressed => {
                next_state.set(AppState::MainMenu);
            }
            _ => {}
        }
    }
}

fn handle_replay_button(
    query: Query<(&Interaction, &ReplayButton)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let Ok((interaction, _replay_button)) = query.single() {
        match *interaction {
            Interaction::Pressed => {
                next_state.set(GameState::Replay);
            }
            _ => {}
        }
    };
}
