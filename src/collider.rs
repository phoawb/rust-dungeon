use sfml::system::Vector2f;

#[derive(Debug)]
pub struct Collider {
    size: Vector2f,
    position: Vector2f,
    hp: i32,
}

impl Collider {
    pub fn new(size: Vector2f, position: Vector2f, hp: Option<i32>) -> Self {
        Collider {
            size,
            position,
            hp: hp.unwrap_or(0),
        }
    }

    pub fn check_collision(&mut self, other: &mut Collider, mut push: f32) -> bool {
        let other_position: Vector2f = other.get_position();
        let other_half_size: Vector2f = other.get_half_size();
        let self_position: Vector2f = self.get_position();
        let self_half_size: Vector2f = self.get_half_size();

        let delta = other_position - self_position;
        let intersect = Vector2f::new(
            delta.x.abs() - (other_half_size.x + self_half_size.x),
            delta.y.abs() - (other_half_size.y + self_half_size.y),
        );

        if self.position.y.is_nan() {
            panic!("y became undefined...");
        }
        if intersect.x < 0.0 && intersect.y < 0.0 {
            push = push.max(0.0).min(1.0);
            if intersect.x > intersect.y {
                if delta.x > 0.0 {
                    self.update(intersect.x * (1.0 - push), 0.0);
                    other.update(-intersect.x * push, 0.0);
                } else {
                    self.update(-intersect.x * (1.0 - push), 0.0);
                    other.update(intersect.x * push, 0.0);
                }
            } else if delta.y > 0.0 {
                self.update(0.0, intersect.y * (1.0 - push));
                other.update(0.0, -intersect.y * push);
            } else {
                self.update(0.0, -intersect.y * (1.0 - push));
                other.update(0.0, intersect.y * push);
            }
            return true;
        }
        false
    }

    pub fn get_position(&self) -> Vector2f {
        self.position
    }

    pub fn set_position(&mut self, position: Vector2f) {
        self.position = position;
    }

    fn get_half_size(&self) -> Vector2f {
        self.size / 2.0
    }

    pub fn take_damage(&mut self, damage: i32) {
        self.hp -= damage;
    }

    pub fn get_hp(&self) -> i32 {
        self.hp
    }

    fn update(&mut self, delta_x: f32, delta_y: f32) {
        self.position = Vector2f::new(self.position.x + delta_x, self.position.y + delta_y);
    }
}
