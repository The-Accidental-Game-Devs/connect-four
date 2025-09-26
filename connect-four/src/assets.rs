use bevy::prelude::Handle;
use bevy::prelude::Image;
use bevy::prelude::Resource;

#[derive(Resource)]
pub struct Assets {
    pub board: Handle<Image>,
    pub red_piece: Handle<Image>,
    pub yellow_piece: Handle<Image>,
}
