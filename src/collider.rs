use std::os::unix::process;

use sfml::system::Vector2f;

#[derive(Debug)]
pub struct Collider {
    size: Vector2f,
    position: Vector2f,
}

impl Collider {
    pub fn new(size: Vector2f, position: Vector2f) -> Self {
        Collider { size, position }
    }

    pub fn check_collision(&mut self, other: &mut Collider, mut push: f32) -> bool {
        let other_position: Vector2f = other.get_position();
        let other_half_size: Vector2f = other.get_half_size();
        let self_position: Vector2f = self.get_position();
        let self_half_size: Vector2f = self.get_half_size();

        println!("player pos is: {self_position:?}");
        println!("other pos is: {other_position:?}");
        let delta = other_position - self_position;
        println!("Delta is: {delta:?}");
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
                    println!("x > 0: other pos = {other_position:?}");
                } else {
                    self.update(-intersect.x * (1.0 - push), 0.0);
                    other.update(intersect.x * push, 0.0);
                    println!("else x > 0: other pos = {other_position:?}");
                }
            } else if delta.y > 0.0 {
                self.update(0.0, intersect.y * (1.0 - push));
                other.update(0.0, -intersect.y * push);
                println!("y > 0: other pos = {other_position:?}");
            } else {
                self.update(0.0, -intersect.y * (1.0 - push));
                other.update(0.0, intersect.y * push);
                println!("else y > 0: other pos = {other_position:?}");
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

    fn update(&mut self, delta_x: f32, delta_y: f32) {
        self.position = Vector2f::new(self.position.x + delta_x, self.position.y + delta_y);
    }
}
