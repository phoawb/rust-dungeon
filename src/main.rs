mod room;
mod texture_storage;
mod tile;

use sfml::graphics::*;
use sfml::system::*;
use sfml::window::*;
use texture_storage::{TextureIdentifiers, TextureStorage};
mod animation;
mod player;
use player::Player;
mod map;
use crate::collision_manager::collision_w_walls;
use crate::collision_manager::projectile_collision_w_walls;
use crate::enemies::enemy::spawn_enemies;
use crate::projectile::Projectile;
use map::Map;
mod collision_manager;
use collision_manager::player_collision_w_walls;
use rust_dungeon::VIEW_SIZE;
mod enemies;
mod projectile;

const WIDTH: u32 = 768;
const HEIGHT: u32 = 480;

fn resize_view(window: &RenderWindow, view: &mut View) {
    let aspect_ratio: f32 = window.size().x as f32 / window.size().y as f32;
    println!("Aspect ratio is: {aspect_ratio}");
    let new_size = Vector2f {
        x: aspect_ratio * VIEW_SIZE.y, //* 9.0,
        y: VIEW_SIZE.y,                // * 9.0,
    };
    view.set_size(new_size);
}

fn main() {
    let mut window = sfml::graphics::RenderWindow::new(
        (WIDTH, HEIGHT),
        "Rust Dungeon",
        Style::DEFAULT,
        &Default::default(),
    );
    window.set_vertical_sync_enabled(true);

    let center = Vector2f {
        x: VIEW_SIZE.x / 2.0,
        y: VIEW_SIZE.y / 2.0,
    };

    let mut main_view = View::new(center, VIEW_SIZE * 9.0);
    println!("{:?}", main_view.center());
    main_view.set_center(VIEW_SIZE * 9.0 / 2.0);

    let mut texture_storage = TextureStorage::new();
    texture_storage.load(TextureIdentifiers::Tile, "textures/tile_sheet.png");
    texture_storage.load(TextureIdentifiers::Player, "textures/Pitaya.png");
    texture_storage.load(TextureIdentifiers::Demon, "textures/Demon.png");
    texture_storage.load(TextureIdentifiers::Necromancer, "textures/necromancer.png");
    texture_storage.load(
        TextureIdentifiers::Projectile,
        "textures/water_projectile.png",
    );

    let mut map = Map::from(Vector2 { x: 9, y: 9 });
    map.start(None);
    let spawn = map.get_spawn();
    let position: Vector2<f32> = Vector2 {
        x: spawn.x as f32 * VIEW_SIZE.x + VIEW_SIZE.x / 2.0,
        y: spawn.y as f32 * VIEW_SIZE.y + VIEW_SIZE.y / 2.0,
    };

    let mut player = Player::from(position);
    let mut player_projectiles: Vec<Projectile> = Vec::new();
    main_view.set_size(VIEW_SIZE);
    main_view.set_center(player.get_position());
    //let demon: Demon = Demon::new(position);
    //let necromancer: Necromancer = Necromancer::new(position);
    //let mut enemies: Vec<Box<dyn Enemy>> = vec![Box::new(demon), Box::new(necromancer)];
    let mut map_enemies = spawn_enemies(map.get_room_centers());
    loop {
        // events
        while let Some(ev) = window.poll_event() {
            match ev {
                Event::Closed => {
                    window.close();
                    return;
                }
                Event::Resized {
                    width: _,
                    height: _,
                } => {
                    resize_view(&window, &mut main_view);
                    println!("Resize event activated!")
                }
                Event::MouseButtonPressed { button, x, y } => {
                    //player.shoot();
                    let mouse_coords = window.map_pixel_to_coords(Vector2i { x, y }, &main_view);
                    player_projectiles.push(player.shoot(mouse_coords));
                    println!("{button:?}");
                    println!("x: {x}, y: {y}");
                    println!("moouse coords are: {mouse_coords:?}");
                    println!("player position is: {:?}", player.get_position());
                    println!("Mouse button was pressed!")
                }
                _ => {}
            }
        }

        let mut active_room = map.get_active_room();
        // drawing
        window.clear(Color::BLACK);
        window.set_view(&main_view);
        let mut upper_left_corner_coordinates = Vector2f::new(
            active_room.x as f32 * VIEW_SIZE.x,
            active_room.y as f32 * VIEW_SIZE.y,
        );
        player.update();
        player.set_position(player_collision_w_walls(
            player.get_position(),
            upper_left_corner_coordinates,
            map.get_active_room_doors(),
        ));
        // update the view to show the room the player is in
        map.set_active_room(player.get_position());
        active_room = map.get_active_room();
        upper_left_corner_coordinates = Vector2f::new(
            active_room.x as f32 * VIEW_SIZE.x,
            active_room.y as f32 * VIEW_SIZE.y,
        );
        let active_room_index = map.get_active_room_index();
        let new_center = Vector2f::new(
            active_room.x as f32 * VIEW_SIZE.x + VIEW_SIZE.x / 2.0,
            active_room.y as f32 * VIEW_SIZE.y + VIEW_SIZE.y / 2.0,
        );
        main_view.set_center(new_center);
        // update non-player entities
        let player_position = player.get_position();
        for enemy in map_enemies[active_room_index].iter_mut() {
            if !is_entity_in_active_room(enemy.get_position(), upper_left_corner_coordinates) {
                continue;
            }
            enemy.update(player_position);
            enemy.set_position(collision_w_walls(
                enemy.get_position(),
                upper_left_corner_coordinates,
            ))
        }

        // draw everything
        map.draw(&mut window, texture_storage.get(TextureIdentifiers::Tile));
        for i in (0..player_projectiles.len()).rev() {
            player_projectiles[i].update();
            if projectile_collision_w_walls(
                player_projectiles[i].get_position(),
                upper_left_corner_coordinates,
            ) {
                player_projectiles.remove(i);
                continue;
            }
            player_projectiles[i].draw(
                &mut window,
                texture_storage.get(TextureIdentifiers::Projectile),
            )
        }
        player.draw(&mut window, texture_storage.get(TextureIdentifiers::Player));

        /*         println!(
            "Active room center is: {center:?}",
            center = room_centers[active_room_index]
        ); */
        for enemy in map_enemies[active_room_index].iter_mut() {
            /*             if !is_entity_in_active_room(enemy.get_position(), upper_left_corner_coordinates) {
                continue;
            } */
            //println!("Enemy position is: {pos:?}", pos = enemy.get_position());
            enemy.draw(&mut window, texture_storage.get(enemy.get_identifier()))
        }
        window.display();
    }
}

fn is_entity_in_active_room(position: Vector2f, upper_left_corner_coordinates: Vector2f) -> bool {
    if position.x < upper_left_corner_coordinates.x
        || position.x > upper_left_corner_coordinates.x + VIEW_SIZE.x
    {
        return false;
    }
    if position.y < upper_left_corner_coordinates.y
        || position.y > upper_left_corner_coordinates.y + VIEW_SIZE.y
    {
        return false;
    }
    true
}

// TODO NEXT:

/* TODO For collision:
* Create a vector of enemy bullets & player bullets that live in main
* The player & enemy update function take that vector as an input argument
* All enemy bullets live in 1 vector
*/

/* TODO  ENEMIES:
* Initialise a vector of enemy vectors with generics
* each enemy type is defined in its own file
* Only draw the enemies with the same index as the number of active rooms
 */

/* TODO MAP & ROOMS
* Implement minimap
 * when press E toggle view size
 * stop updating shit
 * display score
 * implement spawning per room
 */

/* TODO COLLISIONS
 * Hadle collisions globally instead of in the rooms
 * Handle player & Wall collisions with coordinate bounds
 * Handle Enemy & -||-
 * Handle player & enemy collisions
 * Handle player & enemy projectile collisions
 * Handle enemy & player projectile collissions
 */
