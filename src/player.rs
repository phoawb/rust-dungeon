use crate::{texture_storage::TextureIdentifiers, Animation};
use sfml::{
    graphics::*,
    system::{Vector2f, Vector2i},
    SfBox,
};

#[derive(Debug)]
pub struct Player {
    position: Vector2f,
    size: Vector2f,
    origin: Vector2f,
    animation: Animation,
}

impl Player {
    //Empty constructor
    /*     pub fn new() -> Tile {
        // Size of one tile always set
        let size = Vector2f { x: 70.0, y: 70.0 };
        Tile {
            position: Vector2f { x: 0.0, y: 0.0 },
            uv_rect: IntRect::new(0, 0, 16, 16),
            size,
            origin: size / 2.0,
        }
    } */

    // Constructor
    pub fn from(position: Vector2f, image_count: Vector2i) -> Player {
        let size = Vector2f { x: 70.0, y: 70.0 };
        Player {
            position,
            size,
            origin: size / 2.0,
            animation: Animation::from(TextureIdentifiers::Player, image_count, 0.2),
        }
    }

    pub fn create_body(&self) -> RectangleShape {
        let mut body = RectangleShape::new();
        body.set_size(self.size);
        body.set_position(self.position);
        body.set_origin(self.origin);
        body.set_texture_rect(&self.animation.get_uv_rect());
        body
    }

    pub fn draw(&self, window: &mut RenderWindow, texture: &SfBox<Texture>) {
        let mut body = self.create_body();
        body.set_scale(Vector2f { x: 1.1, y: 1.1 });
        body.set_texture(texture, false);
        window.draw(&body);
    }
}
