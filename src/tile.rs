use sfml::{graphics::*, system::Vector2f, SfBox};

#[derive(Debug)]
pub struct Tile {
    //pub body: RectangleShape<'a>,
    //hitbox: RectangleShape<'a>,
    position: Vector2f,
    uv_rect: IntRect,
    size: Vector2f,
    origin: Vector2f,
}

impl Tile {
    //Empty constructor
    pub fn new() -> Tile {
        // Size of one tile always set
        let size = Vector2f { x: 32.0, y: 32.0 };
        Tile {
            position: Vector2f { x: 0.0, y: 0.0 },
            uv_rect: IntRect::new(0, 0, 16, 16),
            size,
            origin: size / 2.0,
        }
    }

    // Constructor
    pub fn from(position: Vector2f, image_count: Vector2f) -> Tile {
        let mut tile = Tile::new();
        tile.uv_rect.left = image_count.x as i32 * tile.uv_rect.width;
        tile.uv_rect.top = image_count.y as i32 * tile.uv_rect.height;
        tile.position = position;
        tile
    }

    pub fn create_body(&mut self) -> RectangleShape {
        let mut body = RectangleShape::new();
        body.set_size(self.size);
        body.set_position(self.position);
        body.set_origin(self.origin);
        body.set_texture_rect(&self.uv_rect);
        body
    }

    pub fn draw(&mut self, window: &mut RenderWindow, texture: &SfBox<Texture>) {
        let mut body = self.create_body();
        body.set_texture(texture, false);
        window.draw(&body);
    }
}
