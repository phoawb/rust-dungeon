use rust_dungeon::Body;
use sfml::{
    graphics::*,
    system::{Vector2f, Vector2i},
    SfBox,
};

use crate::animation::Animation;

pub struct Demon {
    hp: i32,
    //damage: i32,
    position: Vector2f,
    size: Vector2f,
    origin: Vector2f,
    row: i32,
    animation: Animation,
    speed: f32,
    facing_right: bool,
    //points: i32,
}

impl Demon {
    pub fn from(position: Vector2f) -> Demon {
        let size = Vector2f { x: 32.0, y: 36.0 };
        let image_count: Vector2i = Vector2i { x: 8, y: 1 };
        let switch_time: f32 = 0.2;
        Demon {
            hp: 60,
            position,
            size,
            origin: size / 2.0,
            row: 0,
            animation: Animation::from(
                crate::texture_storage::TextureIdentifiers::Demon,
                image_count,
                switch_time,
            ),
            speed: 2.0,
            facing_right: false,
        }
    }

    pub fn draw(&mut self, window: &mut RenderWindow, texture: &SfBox<Texture>) {
        let mut body = self.create_body(
            self.size,
            self.position,
            self.origin,
            self.animation.get_uv_rect(),
        );
        body.set_scale(Vector2f { x: 2.0, y: 2.0 });
        body.set_texture(texture, false);
        window.draw(&body);
    }
}

impl Body for Demon {}
