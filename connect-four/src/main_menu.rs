use crate::assets::Assets;
use crate::states::AppState;
use crate::ui_settings::*;
use bevy::prelude::*;

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
    }
}

fn setup(mut commands: Commands, assets: Res<Assets>) {
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
            StateScoped(AppState::MainMenu),
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
                TextColor(Color::BLACK),
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
                        ..default()
                    },
                    BackgroundColor(Color::Srgba(Srgba::from(
                        Srgba::hex(PRIMARY_COLOR).unwrap(),
                    ))),
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
