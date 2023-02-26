use rand::{thread_rng, Rng};
use sfml::{
    graphics::{RenderWindow, Texture},
    system::Vector2f,
    SfBox,
};
use strum::EnumCount;
use strum_macros::{EnumCount, EnumIter, FromRepr};

use crate::texture_storage::TextureIdentifiers;

use super::{demon::Demon, necromancer::Necromancer};

#[derive(Debug, EnumIter, FromRepr, EnumCount)]
enum EnemyType {
    Demon,
    Necromancer,
}

pub trait Enemy {
    fn new(position: Vector2f) -> Self
    where
        Self: Sized;

    fn update(&mut self, player_position: Vector2f);

    fn get_identifier(&self) -> TextureIdentifiers;

    fn set_position(&mut self, position: Vector2f);
    fn get_position(&self) -> Vector2f;

    fn draw(&mut self, window: &mut RenderWindow, texture: &SfBox<Texture>);
}

//TODO: allow seed as input argument
pub fn spawn_enemies(room_centers: Vec<Vector2f>) -> Vec<Vec<Box<dyn Enemy>>> {
    let mut map_enemies: Vec<Vec<Box<dyn Enemy>>> = Vec::new();
    let mut rng = thread_rng();
    let enemy_type_count = EnemyType::COUNT;
    let spawn_offset = Vector2f::new(256.0, 112.0);

    for i in 0..room_centers.len() {
        let room_enemies: Vec<Box<dyn Enemy>> = Vec::new();
        //let initial_spawn = room_centers[i] - spawn_offset;
        map_enemies.push(room_enemies);
        let enemy_amount = rng.gen_range(3..=5);
        for _j in 0..enemy_amount {
            let spawn = room_centers[i]
                + Vector2f::new(
                    spawn_offset.x * rng.gen_range(-1.0..=1.0),
                    spawn_offset.y * rng.gen_range(-1.0..1.0),
                );
            let enemy_index = rng.gen_range(0..enemy_type_count);
            println!("enemy index is: {enemy_index}");
            let enemy_type = EnemyType::from_repr(enemy_index).unwrap_or(EnemyType::Demon);
            let enemy: Box<dyn Enemy> = match enemy_type {
                EnemyType::Demon => Box::new(Demon::new(spawn)),
                EnemyType::Necromancer => Box::new(Necromancer::new(spawn)),
            };
            map_enemies[i].push(enemy);
        }
    }
    map_enemies
}
