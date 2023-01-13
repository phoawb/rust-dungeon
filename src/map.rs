use rust_dungeon::CardinalDirections;
use sfml::{
    graphics::{RenderWindow, Texture},
    system::Vector2,
    SfBox,
};
use strum::IntoEnumIterator;

use crate::room::Room;

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
        let middle_room_coordinates: Vector2<usize> = Vector2 {
            x: self.grid_size.x / 2,
            y: self.grid_size.y / 2,
        };
        // 1. Start by initializing the first room at a random (read: middle) position on the grid.
        // 2. Create a stack to hold the current position and all the positions of previously visited
        // rooms, and push the starting position onto the stack.
        let mut grid_stack: Vec<Vector2<usize>> = vec![middle_room_coordinates];
        self.taken_positions.push(middle_room_coordinates);
        while self.taken_positions.len() < self.number_of_rooms {
            // 3. pop the top position from the stack and use it as the
            // current position. Mark the current position as visited.
            //TODO: PROPERLY HANDLE UNWRAP CALL LMAO!
            let current_room_coordiantes = grid_stack.pop().unwrap();
            /* 4. For each of the four cardinal directions (up, down, left, right) check if the neighboring
            cell is within the grid boundary and is not visited, if it is then:
                a. Create a new room in that direction
                b. push the new position to the stack */
            for direction in CardinalDirections::iter() {
                let new_coordinates = direction.get_direction_coordinates(current_room_coordiantes);
                if new_coordinates.x == 5 && new_coordinates.y == 4 {
                    println!("{:?}", self.taken_positions.contains(&new_coordinates));
                }
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
        println!("After everything we get the taken positions:");
        println!("{:?}", self.taken_positions);
        println!("The length is: {:?}", self.taken_positions.len());
    }

    //fn set_room_doors(&mut self) {}

    pub fn start(&mut self) {
        self.create_rooms();
        //self.set_room_doors();
    }

    pub fn draw(&self, window: &mut RenderWindow, texture: &SfBox<Texture>) {
        for coordinates in &self.taken_positions {
            self.rooms[coordinates.x][coordinates.y].draw(window, texture);
        }
        //println!("count is actually: {}!", count);
    }
}
