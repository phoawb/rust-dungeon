use crate::room::Room;
use rand::{random, thread_rng, Rng};
use rand::{rngs::StdRng, SeedableRng};
use rust_dungeon::CardinalDirections;
use sfml::{
    graphics::{RenderWindow, Texture},
    system::Vector2,
    SfBox,
};
use strum::IntoEnumIterator;

#[derive(Debug)]
pub struct Map {
    grid_size: Vector2<usize>,
    rooms: Vec<Vec<Room>>,
    taken_positions: Vec<Vector2<usize>>,
    number_of_rooms: usize,
}

impl Map {
    pub fn from(grid_size: Vector2<usize>) -> Map {
        let number_of_rooms = 30;
        if grid_size.x * grid_size.y < number_of_rooms {
            // TODO: ABSTRACT AWAY THIS PANIC CALL LMAO
            panic!("The grid must contain at least 30 rooms!");
        }
        Map {
            grid_size,
            rooms: Vec::new(),
            taken_positions: Vec::new(),
            number_of_rooms,
        }
    }

    fn create_rooms(&mut self) {
        // fill the entire grid with tmp rooms,
        // but don't ackknowledge them
        for x in 0..self.grid_size.x {
            self.rooms.push(Vec::new());
            for y in 0..self.grid_size.y {
                self.rooms[x].push(Room::from(Vector2 {
                    x: 768.0 * x as f32,
                    y: 480.0 * y as f32,
                }));
            }
        }

        //TODO: PROPERLY HANDLE USER HAVING CHOSEN SEED OR NOT
        let tmp_seed: u64 = random();
        println!("The seed is: {}", tmp_seed);
        let mut rng = StdRng::seed_from_u64(tmp_seed);

        //TODO: MAKE THE START RANDOM INSTEAD
        let middle_room_coordinates: Vector2<usize> = Vector2 {
            x: self.grid_size.x / 2,
            y: self.grid_size.y / 2,
        };

        let random_x_coord: usize = rng.gen_range(0..self.grid_size.x);
        let random_y_coord: usize = rng.gen_range(0..self.grid_size.y);
        let random_starting_coordinates = Vector2 {
            x: random_x_coord,
            y: random_y_coord,
        };
        println!("The random start is: {:?}", random_starting_coordinates);
        // 1. Start by initializing the first room at a random (read: middle) position on the grid.
        // 2. Create a stack to hold the current position and all the positions of previously visited
        // rooms, and push the starting position onto the stack.
        let mut grid_stack: Vec<Vector2<usize>> = vec![random_starting_coordinates];
        self.taken_positions.push(random_starting_coordinates);
        while self.taken_positions.len() < self.number_of_rooms {
            // 3. pop a random position from the stack and use it as the
            // current position. Mark the current position as visited.
            let random_stack_index = rng.gen_range(0..grid_stack.len());
            //TODO: PROPERLY HANDLE UNWRAP CALL LMAO!
            let current_room_coordiantes = grid_stack.remove(random_stack_index);
            /* 4. For each of the four cardinal directions (up, down, left, right) check if the neighboring
            cell is within the grid boundary and is not visited, if it is then:
                a. Create a new room in that direction
                b. push the new position to the stack */
            for direction in CardinalDirections::iter() {
                // Safety to prevent subtraction with zero cases
                if current_room_coordiantes.x == 0 && matches!(direction, CardinalDirections::Left)
                    || current_room_coordiantes.y == 0
                        && matches!(direction, CardinalDirections::Up)
                {
                    continue;
                }
                let new_coordinates = direction.get_direction_coordinates(current_room_coordiantes);
                if !(!self.taken_positions.contains(&new_coordinates)
                    && new_coordinates.x < self.grid_size.x
                    && new_coordinates.y < self.grid_size.y)
                {
                    continue;
                }
                grid_stack.push(new_coordinates);
                self.taken_positions.push(new_coordinates);
            }
        }
    }

    fn set_room_doors(&mut self) {
        for coordinate in &self.taken_positions {
            for direction in CardinalDirections::iter() {
                // Safety to prevent subtraction with zero cases
                if coordinate.x == 0 && matches!(direction, CardinalDirections::Left)
                    || coordinate.y == 0 && matches!(direction, CardinalDirections::Up)
                {
                    continue;
                }
                if self
                    .taken_positions
                    .contains(&direction.get_direction_coordinates(*coordinate))
                {
                    self.rooms[coordinate.x][coordinate.y].set_door(direction);
                }
            }
        }
    }

    pub fn start(&mut self) {
        self.create_rooms();
        self.set_room_doors();
    }

    pub fn draw(&self, window: &mut RenderWindow, texture: &SfBox<Texture>) {
        for coordinates in &self.taken_positions {
            self.rooms[coordinates.x][coordinates.y].draw(window, texture);
        }
        //println!("count is actually: {}!", count);
    }
}
