use std::f32::consts::PI;

use rust_dungeon::Body;
use sfml::{
    graphics::{RenderTarget, RenderWindow, Shape, Texture, Transformable},
    system::{Vector2f, Vector2i},
    SfBox,
};

use crate::{animation::Animation, collider::Collider, texture_storage::TextureIdentifiers};

#[derive(Debug)]
pub struct Projectile {
    size: Vector2f,
    direction: Vector2f,
    origin: Vector2f,
    animation: Animation,
    speed: f32,
    collider: Collider,
    collided: bool,
    damage: i32,
}

//TODO: implement default for projectile
impl Projectile {
    pub fn new(position: Vector2f, size: Vector2f, direction: Vector2f) -> Projectile {
        let origin = size / 2.0;
        let image_count = Vector2i::new(17, 1);
        let switch_time = 0.2;
        let speed = 5.0;
        Projectile {
            size,
            direction,
            origin,
            animation: Animation::from(TextureIdentifiers::Projectile, image_count, switch_time),
            speed,
            collider: Collider::new(size, position, None),
            collided: false,
            damage: 20,
        }
    }
    pub fn update(&mut self) {
        self.collider
            .set_position(self.get_position() + self.direction * self.speed);
        let row = 0;
        let face_right = true;
        self.animation.update(row, face_right)
    }

    pub fn get_position(&self) -> Vector2f {
        self.collider.get_position()
    }

    pub fn get_collider(&mut self) -> &mut Collider {
        &mut self.collider
    }

    pub fn set_collided(&mut self, collided: bool) {
        self.collided = collided;
    }

    pub fn has_collided(&self) -> bool {
        self.collided
    }

    pub fn get_damage(&self) -> i32 {
        self.damage
    }

    pub fn draw(&self, window: &mut RenderWindow, texture: &SfBox<Texture>) {
        let mut body = self.create_body(
            self.size,
            self.collider.get_position(),
            self.origin,
            self.animation.get_uv_rect(),
        );
        let angle: f32 = if self.direction.x < 0.0 {
            180.0 - (180.0 / PI) * ((-1.0 * self.direction.y) / self.direction.x).atan()
        } else {
            -1.0 * (180.0 / PI) * ((-1.0 * self.direction.y) / self.direction.x).atan()
        };
        body.set_rotation(angle);
        body.set_scale(Vector2f { x: 1.1, y: 1.1 });
        body.set_texture(texture, false);
        window.draw(&body);
    }
}

impl Body for Projectile {}
