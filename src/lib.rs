use sfml::system::Vector2;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter)]
pub enum CardinalDirections {
    Up,
    Down,
    Left,
    Right,
}

impl CardinalDirections {
    pub fn get_direction_coordinates(&self, coordinates: Vector2<usize>) -> Vector2<usize> {
        println!("{:?}", coordinates);
        match self {
            CardinalDirections::Up => Vector2 {
                x: coordinates.x,
                y: std::cmp::max(coordinates.y, 1) - 1,
            },
            CardinalDirections::Down => Vector2 {
                x: coordinates.x,
                y: coordinates.y + 1,
            },
            CardinalDirections::Right => Vector2 {
                x: coordinates.x + 1,
                y: coordinates.y,
            },
            CardinalDirections::Left => Vector2 {
                x: std::cmp::max(coordinates.x, 1) - 1,
                y: coordinates.y,
            },
        }
    }
}
