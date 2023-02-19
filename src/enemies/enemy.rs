use sfml::{
    graphics::{RenderWindow, Texture},
    system::Vector2f,
    SfBox,
};

pub trait Enemy {
    fn new(position: Vector2f) -> Self;

    fn update(&mut self, player_position: Vector2f);

    fn draw(&mut self, window: &mut RenderWindow, texture: &SfBox<Texture>);
}
