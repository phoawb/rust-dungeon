use rust_dungeon::Body;
use sfml::{
    graphics::*,
    system::{Vector2f, Vector2i},
    SfBox,
};

use crate::animation::Animation;

use super::enemy::Enemy;

pub struct Demon {
    hp: i32,
    //damage: i32,
    position: Vector2f,
    size: Vector2f,
    origin: Vector2f,
    row: i32,
    animation: Animation,
    speed: f32,
    face_right: bool,
    //points: i32,
}

impl Enemy for Demon {
    fn new(position: Vector2f) -> Self {
        let size = Vector2f { x: 32.0, y: 36.0 };
        let image_count: Vector2i = Vector2i { x: 8, y: 1 };
        let switch_time: f32 = 0.2;
        Demon {
            hp: 60,
            //damage: 10
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
            face_right: false,
        }
    }

    //The demon chases the player in order to hurt them
    fn update(&mut self, player_position: Vector2f) {
        let x_dif: f32 = player_position.x - self.position.x;
        let y_dif: f32 = player_position.y - self.position.y;
        let mut movement = Vector2f { x: x_dif, y: y_dif };
        self.face_right = x_dif >= 0.0;
        self.animation.update(self.row, self.face_right);
        if movement.x == 0.0 && movement.y == 0.0 {
            return;
        }
        movement.x /= (x_dif * x_dif + y_dif * y_dif).sqrt();
        movement.y /= (x_dif * x_dif + y_dif * y_dif).sqrt();
        movement *= self.speed;
        self.position += movement;
    }

    fn draw(&mut self, window: &mut RenderWindow, texture: &SfBox<Texture>) {
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
