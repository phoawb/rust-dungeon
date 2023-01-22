use sfml::system::Vector2;
use strum_macros::{EnumCount, EnumIter, FromRepr};
#[derive(Debug, EnumIter, Copy, Clone)]
pub enum CardinalDirection {
    Up,
    Down,
    Left,
    Right,
}
use rand::Rng;
use rand::{rngs::StdRng, SeedableRng};
use std::collections::HashMap;

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

#[derive(Debug, EnumIter, Copy, Clone, FromRepr, EnumCount)]
pub enum RoomColor {
    Brown,
    Red,
    Pink,
    Blue,
    Green,
    Purple,
}

pub fn get_room_colors(
    adjacency_list: &HashMap<usize, Vec<usize>>,
    num_colors: usize,
    seed: u64,
) -> HashMap<usize, usize> {
    // Create a vector to store the degree of each room
    let mut degrees: Vec<usize> = (0..adjacency_list.len())
        .map(|i| adjacency_list[&i].len())
        .collect();

    let mut indices: Vec<usize> = (0..degrees.len()).collect();

    indices.sort_by(|&i1, &i2| degrees[i2].cmp(&degrees[i1]));
    // Sort the rooms in descending order of their degree
    degrees.sort_by(|a, b| b.cmp(a));

    // Create a vector to store the color of each room
    let mut colors: Vec<usize> = vec![0; adjacency_list.len()];

    // Create a vector to store the available colors for each room
    let mut available_colors: Vec<Vec<bool>> = vec![vec![true; num_colors]; adjacency_list.len()];

    // Assign the first color to the highest degree room
    colors[indices[0]] = 0;

    // Mark the first color as unavailable for the adjacent rooms
    for &adjacent_room in &adjacency_list[&indices[0]] {
        available_colors[adjacent_room][0] = false;
    }

    // Set the rng outside the forlop to avoid reassigning it
    let mut rng = StdRng::seed_from_u64(seed);

    // Iterate through the remaining rooms
    for i in 1..indices.len() {
        let room = indices[i];

        // Find a random color that is not used by any of its adjacent rooms
        let mut color = rng.gen_range(0..num_colors);
        println!("firsst color is: {}", color);
        while !available_colors[room][color] {
            color = rng.gen_range(0..num_colors);
            println!("Random color is: {}", color);
        }

        // Assign the color to the room
        colors[room] = color;

        // Mark the assigned color as unavailable for the adjacent rooms
        for &adjacent_room in &adjacency_list[&room] {
            available_colors[adjacent_room][color] = false;
        }
    }
    let color_map = colors
        .into_iter()
        .enumerate()
        .map(|(i, c)| (i, c))
        .collect();
    color_map
}
