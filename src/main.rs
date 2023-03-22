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
use crate::collider::Collider;
use crate::collision_manager::collision_w_walls;
use crate::collision_manager::projectile_collision_w_walls;
use crate::enemies::enemy::spawn_enemies;
use crate::projectile::Projectile;
use map::Map;
mod collision_manager;
use collision_manager::player_collision_w_walls;
use rust_dungeon::VIEW_SIZE;
mod collider;
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
    let input_seed: u64 = 14348464890032967579;
    map.start(Some(input_seed));
    let spawn = map.get_spawn();
    let position: Vector2<f32> = Vector2 {
        x: spawn.x as f32 * VIEW_SIZE.x + VIEW_SIZE.x / 2.0,
        y: spawn.y as f32 * VIEW_SIZE.y + VIEW_SIZE.y / 2.0,
    };

    let mut player = Player::from(position);
    let mut player_projectiles: Vec<Projectile> = Vec::new();
    main_view.set_size(VIEW_SIZE);
    main_view.set_center(player.get_position());
    let mut map_enemies = spawn_enemies(map.get_room_centers());
    let mut map_enemy_colliders: Vec<Vec<Collider>> = Vec::new();

    for i in 0..map_enemies.len() {
        map_enemy_colliders.push(Vec::new());
        for j in 0..map_enemies[i].len() {
            let enemy_collider = Collider::new(
                map_enemies[i][j].get_size(),
                map_enemies[i][j].get_position(),
                Some(map_enemies[i][j].get_hp()),
            );
            map_enemy_colliders[i].push(enemy_collider);
        }
    }

    let mut minimap = false;
    let mut time = Clock::start();

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
                Event::MouseButtonPressed { button: _, x, y } => {
                    //player.shoot();
                    let mouse_coords = window.map_pixel_to_coords(Vector2i { x, y }, &main_view);
                    player_projectiles.push(player.shoot(mouse_coords));
                }
                _ => {}
            }
        }

        if Key::E.is_pressed() && time.elapsed_time().as_seconds() > 0.2 {
            minimap = toggle_minimap(minimap);
            time.restart();
        }

        if minimap {
            main_view.set_size(VIEW_SIZE * 9.0);
            window.set_view(&main_view);
            window.clear(Color::BLACK);
            map.draw_visited_rooms(&mut window, texture_storage.get(TextureIdentifiers::Tile));
            player.draw(&mut window, texture_storage.get(TextureIdentifiers::Player));
            window.display();
            continue;
        } else {
            main_view.set_size(VIEW_SIZE);
        }

        let mut active_room = map.get_active_room();
        // drawing
        window.clear(Color::BLACK);
        window.set_view(&main_view);
        let mut upper_left_corner_coordinates = Vector2f::new(
            active_room.x as f32 * VIEW_SIZE.x,
            active_room.y as f32 * VIEW_SIZE.y,
        );
        let mut active_room_index = map.get_active_room_index();
        player.update();

        for projectile in player_projectiles.iter_mut() {
            projectile.set_collided(projectile_collision_w_walls(
                projectile.get_position(),
                upper_left_corner_coordinates,
            ))
        }

        for i in 0..map_enemies[active_room_index].len() {
            let player_collider = player.get_collider();
            let push: f32 = 1.0;
            player_collider.check_collision(&mut map_enemy_colliders[active_room_index][i], 0.4);
            map_enemies[active_room_index][i]
                .set_position(map_enemy_colliders[active_room_index][i].get_position());
            for projectile in player_projectiles.iter_mut() {
                let projectile_collider = projectile.get_collider();
                if !projectile_collider
                    .check_collision(&mut map_enemy_colliders[active_room_index][i], push)
                {
                    continue;
                }
                projectile.set_collided(true);
                map_enemies[active_room_index][i]
                    .set_position(map_enemy_colliders[active_room_index][i].get_position());
                let projectile_damage = projectile.get_damage();
                map_enemies[active_room_index][i].take_damage(projectile_damage);
                map_enemy_colliders[active_room_index][i].take_damage(projectile_damage);
            }
        }

        map_enemies[active_room_index].retain(|e| e.get_hp() > 0);
        map_enemy_colliders[active_room_index].retain(|c| c.get_hp() > 0);

        player_projectiles.retain(|p| !p.has_collided());
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
        active_room_index = map.get_active_room_index();
        let new_center = Vector2f::new(
            active_room.x as f32 * VIEW_SIZE.x + VIEW_SIZE.x / 2.0,
            active_room.y as f32 * VIEW_SIZE.y + VIEW_SIZE.y / 2.0,
        );
        main_view.set_center(new_center);
        // update non-player entities
        let player_position = player.get_position();

        for i in 0..map_enemies[active_room_index].len() {
            map_enemies[active_room_index][i].update(player_position);
            let enemy_position = map_enemies[active_room_index][i].get_position();
            map_enemies[active_room_index][i].set_position(collision_w_walls(
                enemy_position,
                upper_left_corner_coordinates,
            ));
            map_enemy_colliders[active_room_index][i].set_position(enemy_position);
        }

        // draw everything
        map.draw(&mut window, texture_storage.get(TextureIdentifiers::Tile));
        for projectile in player_projectiles.iter_mut() {
            projectile.update();
            projectile.draw(
                &mut window,
                texture_storage.get(TextureIdentifiers::Projectile),
            )
        }
        player.draw(&mut window, texture_storage.get(TextureIdentifiers::Player));

        for enemy in map_enemies[active_room_index].iter_mut() {
            /*             if !is_entity_in_active_room(enemy.get_position(), upper_left_corner_coordinates) {
                continue;
            } */
            enemy.draw(&mut window, texture_storage.get(enemy.get_identifier()))
        }
        window.display();
    }
}

fn toggle_minimap(minimap: bool) -> bool {
    if minimap {
        return false;
    }
    true
}

// TODO NEXT:

/* TODO for MVP
* have enemies be able to shoot
* player is able to take damage
* player is able to dash
* player has invis for 1 second after entering a new room
* sound (?)
*/

// TODO: The dreaded main menu
/*
* state machine in rust :(((((
 * main menu state
 * game state
 * win state
 * lose state
 */

/* TODO For collision:
* Create a vector of enemy bullets & player bullets that live in main
* The player & enemy update function take that vector as an input argument
* All enemy bullets live in 1 vector
*/

/* TODO MAP & ROOMS
 * display score
 */

/* TODO COLLISIONS
 * Handle player & enemy projectile collisions
 */
