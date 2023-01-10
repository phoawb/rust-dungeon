use crate::texture_storage::TextureIdentifiers;
use sfml::{graphics::IntRect, system::Vector2i};

#[derive(Debug)]
pub struct Animation {
    image_count: Vector2i,
    current_image: Vector2i,
    total_time: f32,
    switch_time: f32,
    uv_rect: IntRect,
}

fn get_texture_size(identifier: TextureIdentifiers) -> Vector2i {
    let texture_size: Vector2i = match identifier {
        TextureIdentifiers::Player => Vector2i { x: 768, y: 1920 },
        TextureIdentifiers::Tile => Vector2i { x: 64, y: 64 }, // should never be used, only here for completeness
    };
    texture_size
}

impl Animation {
    /*     pub fn new() -> Animation {
        Animation {
            image_count: Vector2f { x: 0.0, y: 0.0 },
            current_image: Vector2f { x: 0.0, y: 0.0 },
            total_time: 0.0,
            switch_time: 1.0,
        }
    } */

    pub fn from(
        identifier: TextureIdentifiers,
        image_count: Vector2i,
        switch_time: f32,
    ) -> Animation {
        let texture_size = get_texture_size(identifier);
        Animation {
            image_count,
            current_image: Vector2i { x: 0, y: 0 },
            total_time: 0.0,
            switch_time,
            uv_rect: IntRect::new(
                0,
                0,
                texture_size.x / image_count.x,
                texture_size.y / image_count.y,
            ),
        }
    }

    pub fn update(&mut self, row: i32, face_right: bool) {
        self.current_image.y = row;
        self.total_time += 1.0 / 60.0;

        if self.total_time >= self.switch_time {
            self.total_time -= self.switch_time;
            self.current_image.x += 1;
            if self.current_image.x >= self.image_count.x {
                self.current_image.x = 0;
            }
        }

        self.uv_rect.top = self.current_image.y * self.uv_rect.height;

        if face_right {
            self.uv_rect.left = self.current_image.x * self.uv_rect.width
        } else {
            self.uv_rect.left = (self.current_image.x + 1) * self.uv_rect.width.abs();
            self.uv_rect.width = -self.uv_rect.width.abs();
        }
    }

    pub fn get_uv_rect(&self) -> IntRect {
        self.uv_rect
    }
}
