use sfml::system::Vector2;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter, Copy, Clone)]
pub enum CardinalDirection {
    Up,
    Down,
    Left,
    Right,
}

impl CardinalDirection {
    pub fn get_direction_coordinates(&self, coordinates: Vector2<usize>) -> Vector2<usize> {
        match self {
            CardinalDirection::Up => Vector2 {
                x: coordinates.x,
                y: std::cmp::max(coordinates.y, 1) - 1,
            },
            CardinalDirection::Down => Vector2 {
                x: coordinates.x,
                y: coordinates.y + 1,
            },
            CardinalDirection::Right => Vector2 {
                x: coordinates.x + 1,
                y: coordinates.y,
            },
            CardinalDirection::Left => Vector2 {
                x: std::cmp::max(coordinates.x, 1) - 1,
                y: coordinates.y,
            },
        }
    }
    pub fn get_opposite_direction(&self) -> CardinalDirection {
        match self {
            CardinalDirection::Up => CardinalDirection::Down,
            CardinalDirection::Down => CardinalDirection::Up,
            CardinalDirection::Right => CardinalDirection::Left,
            CardinalDirection::Left => CardinalDirection::Right,
        }
    }
}
