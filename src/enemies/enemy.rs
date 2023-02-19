use sfml::{
    graphics::{RenderWindow, Texture},
    system::Vector2f,
    SfBox,
};

use crate::texture_storage::TextureIdentifiers;

pub trait Enemy {
    fn new(position: Vector2f) -> Self
    where
        Self: Sized;

    fn update(&mut self, player_position: Vector2f);

    fn get_identifier(&self) -> TextureIdentifiers;

    fn set_position(&mut self, position: Vector2f);
    fn get_position(&self) -> Vector2f;

    fn draw(&mut self, window: &mut RenderWindow, texture: &SfBox<Texture>);
}
