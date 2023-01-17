use crate::room::Room;
use rand::{random, Rng};
use rand::{rngs::StdRng, SeedableRng};
use rust_dungeon::CardinalDirection;
use sfml::{
    graphics::{RenderWindow, Texture},
    system::Vector2,
    SfBox,
};
use std::collections::VecDeque;
use strum::IntoEnumIterator;

#[derive(Debug)]
pub struct Map {
    grid_size: Vector2<usize>,
    rooms: Vec<Vec<Room>>,
    taken_positions: Vec<Vector2<usize>>,
    number_of_rooms: usize,
    spawn: Vector2<usize>,
    end: Vector2<usize>,
}

impl Map {
    pub fn from(grid_size: Vector2<usize>) -> Map {
        let number_of_rooms = 30;
        if grid_size.x * grid_size.y < number_of_rooms {
            // TODO: ABSTRACT AWAY THIS PANIC CALL LMAO
            panic!("The grid must contain at least 30 rooms!");
        }
        let spawn: Vector2<usize> = Vector2 { x: 4, y: 4 };
        let end: Vector2<usize> = Vector2 { x: 4, y: 4 };
        Map {
            grid_size,
            rooms: Vec::new(),
            taken_positions: Vec::new(),
            number_of_rooms,
            spawn,
            end,
        }
    }

    fn create_rooms(&mut self, seed: u64, starting_coordinates: Vector2<usize>) {
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
        println!("The seed is: {}", seed);
        let mut rng = StdRng::seed_from_u64(seed);

        //TODO: MAKE THE START RANDOM INSTEAD
        let middle_room_coordinates: Vector2<usize> = Vector2 {
            x: self.grid_size.x / 2,
            y: self.grid_size.y / 2,
        };

        // 1. Start by initializing the first room at a random (read: middle) position on the grid.
        // 2. Create a stack to hold the current position and all the positions of previously visited
        // rooms, and push the starting position onto the stack.
        let mut grid_stack: Vec<Vector2<usize>> = vec![starting_coordinates];
        self.taken_positions.push(starting_coordinates);
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
            for direction in CardinalDirection::iter() {
                // Safety to prevent subtraction with zero cases
                if current_room_coordiantes.x == 0 && matches!(direction, CardinalDirection::Left)
                    || current_room_coordiantes.y == 0 && matches!(direction, CardinalDirection::Up)
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

    // This algorithm can set the same door multiple times,
    // but it isn't worth optimizing currently
    fn set_room_doors(&mut self, seed: u64, probability: f32) {
        let mut rng = StdRng::seed_from_u64(seed);
        for coordinate in &self.taken_positions {
            for direction in CardinalDirection::iter() {
                // Safety to prevent subtraction with zero cases
                if coordinate.x == 0 && matches!(direction, CardinalDirection::Left)
                    || coordinate.y == 0 && matches!(direction, CardinalDirection::Up)
                {
                    continue;
                }
                let neighbouring_coordinate = direction.get_direction_coordinates(*coordinate);
                if !self.taken_positions.contains(&neighbouring_coordinate) {
                    continue;
                }
                if rng.gen_range(0.0..=1.0) < probability {
                    self.rooms[coordinate.x][coordinate.y].set_door(direction);
                    self.rooms[neighbouring_coordinate.x][neighbouring_coordinate.y]
                        .set_door(direction.get_opposite_direction());
                }
            }
        }
        self.fix_closed_rooms(seed);
    }

    fn fix_closed_rooms(&mut self, seed: u64) {
        for coordinate in &self.taken_positions {
            if self.rooms[coordinate.x][coordinate.y].has_doors() {
                continue;
            }
            let mut neighbouring_room_direction: Vec<CardinalDirection> = Vec::new();
            for direction in CardinalDirection::iter() {
                let neighbouring_coordinate = direction.get_direction_coordinates(*coordinate);
                if self.taken_positions.contains(&neighbouring_coordinate) {
                    neighbouring_room_direction.push(direction);
                }
            }
            let mut rng = StdRng::seed_from_u64(seed);
            let random_direction =
                neighbouring_room_direction[rng.gen_range(0..neighbouring_room_direction.len())];
            self.rooms[coordinate.x][coordinate.y].set_door(random_direction);
            let neighbouring_room_coords = random_direction.get_direction_coordinates(*coordinate);
            self.rooms[neighbouring_room_coords.x][neighbouring_room_coords.y]
                .set_door(random_direction.get_opposite_direction());
        }
    }

    pub fn start(&mut self) {
        let seed: u64 = random();
        let probability = 0.55;
        let mut rng = StdRng::seed_from_u64(14348464890032967579);
        let random_x_coord: usize = rng.gen_range(0..self.grid_size.x);
        let random_y_coord: usize = rng.gen_range(0..self.grid_size.y);
        let starting_coordinates = Vector2 {
            x: random_x_coord,
            y: random_y_coord,
        };
        println!("The random start is: {:?}", starting_coordinates);
        self.create_rooms(14348464890032967579, starting_coordinates);
        self.set_room_doors(14348464890032967579, probability);
        self.spawn = self.get_farthest_coordinate(starting_coordinates);
        self.end = self.get_farthest_coordinate(self.spawn);
        println!("spawn is {:?}", self.spawn);
        println!("end is {:?}", self.end);
    }

    //BFS search that returns the last item in the queue
    fn get_farthest_coordinate(&self, starting_coordinates: Vector2<usize>) -> Vector2<usize> {
        let mut visited_rooms: Vec<Vector2<usize>> = Vec::new();
        let mut queue: VecDeque<Vector2<usize>> = VecDeque::new();
        let mut current_room: Vector2<usize> = Vector2 { x: 0, y: 0 };

        visited_rooms.push(starting_coordinates);
        queue.push_back(starting_coordinates);

        while !queue.is_empty() {
            current_room = queue.pop_front().unwrap();

            for direction in CardinalDirection::iter() {
                if self.rooms[current_room.x][current_room.y].get_door(direction) {
                    let neighbouring_room = direction.get_direction_coordinates(current_room);
                    if !visited_rooms.contains(&neighbouring_room) {
                        visited_rooms.push(neighbouring_room);
                        queue.push_back(neighbouring_room);
                    }
                }
            }
        }

        current_room
    }

    pub fn get_spawn(&self) -> Vector2<usize> {
        self.spawn
    }

    pub fn draw(&self, window: &mut RenderWindow, texture: &SfBox<Texture>) {
        for coordinates in &self.taken_positions {
            self.rooms[coordinates.x][coordinates.y].draw(window, texture);
        }
    }
}
