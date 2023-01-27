use crate::room::Room;
use rand::Rng;
use rand::{rngs::StdRng, SeedableRng};
use rust_dungeon::{get_room_colors, CardinalDirection, RoomColor};
use sfml::system::Vector2f;
use sfml::{
    graphics::{RenderWindow, Texture},
    system::Vector2,
    SfBox,
};
use std::collections::HashMap;
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
    active_room: Vector2<usize>,
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
        let active_room: Vector2<usize> = Vector2 { x: 4, y: 4 };
        Map {
            grid_size,
            rooms: Vec::new(),
            taken_positions: Vec::new(),
            number_of_rooms,
            spawn,
            end,
            active_room,
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

        // 1. Start by initializing the first room at a random (read: middle) position on the grid.
        // 2. Create a stack to hold the current position and all the positions of previously visited
        // rooms, and push the starting position onto the stack.
        let mut grid_stack: Vec<Vector2<usize>> = vec![starting_coordinates];
        self.taken_positions.push(starting_coordinates);
        while self.taken_positions.len() < self.number_of_rooms {
            // 3. pop a random position from the stack and use it as the
            // current position. Mark the current position as visited.
            let random_stack_index = rng.gen_range(0..grid_stack.len());
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

    fn set_room_colors(&mut self, colors: HashMap<usize, usize>) {
        for i in 0..self.taken_positions.len() {
            let coordinates = self.taken_positions[i];
            let color_index = colors.get(&i).unwrap();

            let color = match color_index {
                0 => RoomColor::Pink,
                1 => RoomColor::Blue,
                2 => RoomColor::Green,
                3 => RoomColor::Purple,
                _ => RoomColor::Brown,
            };
            self.rooms[coordinates.x][coordinates.y].set_color(color)
        }
        /*         let mut rng = StdRng::seed_from_u64(seed);
        for coordinate in &self.taken_positions {
            //exclude brown (default) & red (only for start and end)
            let random_index = rng.gen_range(2..RoomColor::COUNT);
            let random_color = RoomColor::from_repr(random_index).unwrap_or(RoomColor::Brown);
            self.rooms[coordinate.x][coordinate.y].set_color(random_color);
        } */
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

    pub fn start(&mut self, input_seed: Option<u64>) {
        //let seed: u64 = input_seed.unwrap_or(random());
        let seed: u64 = 14348464890032967579;
        let probability = 0.55;
        let mut rng = StdRng::seed_from_u64(seed);
        let random_x_coord: usize = rng.gen_range(0..self.grid_size.x);
        let random_y_coord: usize = rng.gen_range(0..self.grid_size.y);
        let starting_coordinates = Vector2 {
            x: random_x_coord,
            y: random_y_coord,
        };
        println!("The random start is: {:?}", starting_coordinates);
        self.create_rooms(seed, starting_coordinates);
        self.set_room_doors(seed, probability);
        let adjacency_list = self.get_adjacency_list();
        let num_colors = 4;
        let colors = get_room_colors(&adjacency_list, num_colors, seed);
        self.set_room_colors(colors);
        self.spawn = self.get_farthest_coordinate(starting_coordinates);
        self.active_room = self.spawn;
        self.rooms[self.spawn.x][self.spawn.y].set_color(RoomColor::Red);
        self.end = self.get_farthest_coordinate(self.spawn);
        self.rooms[self.end.x][self.end.y].set_color(RoomColor::Red);
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
                if !self.rooms[current_room.x][current_room.y].get_door(direction) {
                    continue;
                }
                let neighbouring_room = direction.get_direction_coordinates(current_room);
                if !self.taken_positions.contains(&neighbouring_room) {
                    continue;
                }
                if !visited_rooms.contains(&neighbouring_room) {
                    visited_rooms.push(neighbouring_room);
                    queue.push_back(neighbouring_room);
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

    fn get_neighbouring_room_indexes(&self, coordinate: Vector2<usize>) -> Vec<usize> {
        let mut neighbouring_room_indexes: Vec<usize> = Vec::new();
        for direction in CardinalDirection::iter() {
            let neighbouring_room = direction.get_direction_coordinates(coordinate);
            if self.taken_positions.contains(&neighbouring_room) {
                let index = self
                    .taken_positions
                    .iter()
                    .position(|&r| r == neighbouring_room)
                    .unwrap();
                neighbouring_room_indexes.push(index);
            }
        }
        neighbouring_room_indexes
    }

    // 1. adjust adjesensy list so that it just uses numbers (index of taken positions)
    fn get_adjacency_list(&self) -> HashMap<usize, Vec<usize>> {
        let mut adjacency_list: HashMap<usize, Vec<usize>> = HashMap::new();
        for coordinate in &self.taken_positions {
            let neighbouring_room_indexes = self.get_neighbouring_room_indexes(*coordinate);
            // TODO HANDLE UNWRAP CALL LMAO
            let index = self
                .taken_positions
                .iter()
                .position(|&r| r == *coordinate)
                .unwrap();
            adjacency_list.insert(index, neighbouring_room_indexes);
        }
        adjacency_list
    }

    pub fn get_active_room(&self) -> Vector2<usize> {
        self.active_room
    }

    pub fn set_active_room(&mut self, player_position: Vector2f) {
        if self.rooms[self.active_room.x][self.active_room.y].contains_player(player_position) {
            return;
        }
        for direction in CardinalDirection::iter() {
            let neighbouring_room_coordinates =
                direction.get_direction_coordinates(self.active_room);
            if !self
                .taken_positions
                .contains(&neighbouring_room_coordinates)
            {
                continue;
            }
            if !self.rooms[neighbouring_room_coordinates.x][neighbouring_room_coordinates.y]
                .contains_player(player_position)
            {
                continue;
            }
            self.active_room = neighbouring_room_coordinates;
        }
    }
}
