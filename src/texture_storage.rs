use std::collections::HashMap;
extern crate sfml;
use sfml::{graphics::Texture, SfBox};

pub struct TextureStorage {
    texture_map: HashMap<TextureIdentifiers, SfBox<Texture>>,
}

impl TextureStorage {
    pub fn new() -> TextureStorage {
        TextureStorage {
            texture_map: HashMap::new(),
        }
    }

    pub fn load(&mut self, identifier: TextureIdentifiers, filename: &str) {
        // TODO: PROPERLY HANDLE THE UNWRAP CALL LMAO
        let texture = Texture::from_file(filename).unwrap();
        self.texture_map.insert(identifier, texture);
    }

    pub fn get(&self, identifier: TextureIdentifiers) -> &SfBox<Texture> {
        //TODO: PROPERLY HANDLE THE UNWRAP CALL LMAO
        self.texture_map.get(&identifier).unwrap()
    }
}

#[derive(PartialEq, Eq, Hash)]
pub enum TextureIdentifiers {
    Tile,
}
