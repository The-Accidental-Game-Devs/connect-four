use bevy::prelude::Font;
use bevy::prelude::Handle;
use bevy::prelude::Image;
use bevy::prelude::Resource;

#[derive(Resource)]
pub struct Assets {
    pub board: Handle<Image>,
    pub board_border: Handle<Image>,
    pub red_piece: Handle<Image>,
    pub yellow_piece: Handle<Image>,
    pub font: Handle<Font>,
    pub bold_font: Handle<Font>,
}
