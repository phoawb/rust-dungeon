extern crate sfml;

mod room;
mod texture_storage;
mod tile;
use crate::texture_storage::{TextureIdentifiers, TextureStorage};
use room::Room;
use sfml::graphics::*;
use sfml::system::*;
use sfml::window::*;
mod animation;
use crate::animation::Animation;
mod player;
use crate::player::Player;

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
        x: aspect_ratio * VIEW_SIZE.y,
        y: VIEW_SIZE.y,
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

    let mut main_view = View::new(center, VIEW_SIZE);

    let mut rectangle = RectangleShape::new();
    rectangle.set_size(Vector2f::new(100.0, 100.0));
    rectangle.set_fill_color(Color::rgb(255, 0, 100));
    rectangle.set_origin(rectangle.size() / 2.0);
    rectangle.set_position(Vector2f::new(main_view.center().x, main_view.center().y));
    println!("{:?}", rectangle.position());

    let mut texture_storage = TextureStorage::new();
    texture_storage.load(TextureIdentifiers::Tile, "textures/tile_sheet1.png");
    texture_storage.load(TextureIdentifiers::Player, "textures/Pitaya.png");

    let room = Room::from(Vector2f { x: 0.0, y: 0.0 });
    let animation: Animation =
        Animation::from(TextureIdentifiers::Player, Vector2i { x: 4, y: 10 }, 0.2);
    let player = Player::from(Vector2f { x: 384.0, y: 240.0 }, Vector2i { x: 4, y: 10 });
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
        room.draw(&mut window, texture_storage.get(TextureIdentifiers::Tile));
        player.draw(&mut window, texture_storage.get(TextureIdentifiers::Player));
        window.display();
    }
}
