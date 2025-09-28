use crate::states::AppState;
use bevy::prelude::*;

#[derive(Component)]
struct BackButton {}

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup);
        app.add_systems(
            Update,
            handle_back_button.run_if(in_state(AppState::InGame)),
        );
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Start,
                ..default()
            },
            StateScoped(AppState::InGame),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Back"),
                TextColor(Color::BLACK),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                Button {},
                BackButton {},
            ));
        });
}

fn handle_back_button(
    query: Query<(&Interaction, &BackButton)>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, _back_button)) = query.single() {
        match *interaction {
            Interaction::Pressed => {
                next_state.set(AppState::MainMenu);
            }
            _ => {}
        }
    }
}

