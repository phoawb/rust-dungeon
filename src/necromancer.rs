use rust_dungeon::Body;
use sfml::{
    graphics::*,
    system::{Vector2f, Vector2i},
    SfBox,
};

use crate::animation::Animation;

pub struct Necromancer {
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

impl Necromancer {
    pub fn from(position: Vector2f) -> Necromancer {
        let size = Vector2f { x: 16.0, y: 20.0 };
        let image_count: Vector2i = Vector2i { x: 4, y: 1 };
        let switch_time: f32 = 0.2;
        Necromancer {
            hp: 40,
            //damage: 5,
            position,
            size,
            origin: size / 2.0,
            row: 0,
            animation: Animation::from(
                crate::texture_storage::TextureIdentifiers::Necromancer,
                image_count,
                switch_time,
            ),
            speed: 1.0,
            face_right: false,
        }
    }

    //The necromancer tries to keep a set distance from the player
    //in order to shoot them
    pub fn update(&mut self, player_position: Vector2f) {
        let x_dif: f32 = player_position.x - self.position.x;
        let y_dif: f32 = player_position.y - self.position.y;

        let mut movement = Vector2f { x: x_dif, y: y_dif };
        self.face_right = if x_dif < 0.0 { false } else { true };
        self.animation.update(self.row, self.face_right);
        if movement.x == 0.0 && movement.y == 0.0 {
            return;
        }
        movement.x = movement.x / (x_dif * x_dif + y_dif * y_dif).sqrt();
        movement.y = movement.y / (x_dif * x_dif + y_dif * y_dif).sqrt();
        if (x_dif * x_dif + y_dif * y_dif).sqrt() <= 250.0 {
            movement.x = -movement.x;
            movement.y = -movement.y;
        }
        movement *= self.speed;
        self.position += movement;
    }

    pub fn draw(&mut self, window: &mut RenderWindow, texture: &SfBox<Texture>) {
        let mut body = self.create_body(
            self.size,
            self.position,
            self.origin,
            self.animation.get_uv_rect(),
        );
        body.set_scale(Vector2f { x: 2.5, y: 2.5 });
        body.set_texture(texture, false);
        window.draw(&body);
    }
}

impl Body for Necromancer {}
