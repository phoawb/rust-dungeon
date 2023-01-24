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
use map::Map;

const WIDTH: u32 = 768;
const HEIGHT: u32 = 480;
const VIEW_SIZE: Vector2<f32> = Vector2f {
    x: WIDTH as f32,
    y: HEIGHT as f32,
};

fn resize_view(window: &RenderWindow, view: &mut View) {
    let aspect_ratio: f32 = window.size().x as f32 / window.size().y as f32;
    println!("Aspect ratio is: {}", aspect_ratio);
    let new_size = Vector2f {
        x: aspect_ratio * VIEW_SIZE.y * 9.0,
        y: VIEW_SIZE.y * 9.0,
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

    //let room = Room::from(Vector2f { x: 0.0, y: 0.0 });
    let mut map = Map::from(Vector2 { x: 9, y: 9 });
    map.start(None);
    let spawn = map.get_spawn();
    let position: Vector2<f32> = Vector2 {
        x: spawn.x as f32 * VIEW_SIZE.x + VIEW_SIZE.x / 2.0,
        y: spawn.y as f32 * VIEW_SIZE.y + VIEW_SIZE.y / 2.0,
    };
    let image_count = Vector2i { x: 4, y: 10 };
    let mut player = Player::from(position, image_count);
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
                Event::MouseButtonPressed {
                    button: _,
                    x: _,
                    y: _,
                } => {
                    println!("Mouse button was pressed!")
                }
                _ => {}
            }
        }

        // drawing
        window.clear(Color::BLACK);
        window.set_view(&main_view);
        player.update();
        //room.draw(&mut window, texture_storage.get(TextureIdentifiers::Tile));
        map.draw(&mut window, texture_storage.get(TextureIdentifiers::Tile));
        player.draw(&mut window, texture_storage.get(TextureIdentifiers::Player));
        window.display();
    }
}

// TODO NEXT:
// * Implement the biter enemy
// * Implement player collision with walls
// * Implement enemy -||-

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
* Get the number of rooms
* Store the index of the actve room
* Have a method to get the active room
* have a function to set active room based on player location
 */

/* TODO COLLISIONS
 * Hadle collisions globally instead of in the rooms
 * Handle player & Wall collisions with coordinate bounds
 * Handle Enemy & -||-
 * Handle player & enemy collisions
 * Handle player & enemy projectile collisions
 * Handle enemy & player projectile collissions
 */
