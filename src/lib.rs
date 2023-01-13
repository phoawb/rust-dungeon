use std::collections::HashMap;
use std::hash::Hash;
extern crate sfml;
use sfml::{
    graphics::{Font, Texture},
    SfBox,
};

pub trait Resource: Sized {
    fn new_from_file(filename: &str) -> Option<Self>;
}

impl Resource for SfBox<Texture> {
    fn new_from_file(filename: &str) -> Option<Self> {
        Some(Texture::from_file(filename).unwrap())
    }
}

impl Resource for SfBox<Font> {
    fn new_from_file(filename: &str) -> Option<Self> {
        Font::from_file(filename)
    }
}

pub struct ResourceManager<I, R> {
    resource_map: HashMap<I, Box<R>>,
}

impl<I: Eq + Hash, R: Resource> ResourceManager<I, R> {
    pub fn new() -> Self {
        ResourceManager {
            resource_map: HashMap::<I, Box<R>>::new(),
        }
    }

    pub fn load(&mut self, identifier: I, filename: &str) {
        let resource = R::new_from_file(filename).unwrap();
        self.resource_map.insert(identifier, Box::new(resource));
    }

    pub fn get(&self, identifier: I) -> &R {
        match self.resource_map.get(&identifier) {
            Some(resource) => resource,
            None => panic!("Tried to access nonexistant index in resource map"),
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
pub enum TextureIdentifiers {
    Nebula,
    Rocket,
    Tile,
}
#[derive(PartialEq, Eq, Hash)]
pub enum FontIdentifiers {
    Arial,
    Joystix,
}

impl<I: Eq + Hash, R: Resource> Default for ResourceManager<I, R> {
    fn default() -> Self {
        Self::new()
    }
}

pub type TextureManager = ResourceManager<TextureIdentifiers, SfBox<Texture>>;
pub type FontManager = ResourceManager<FontIdentifiers, SfBox<Font>>;

/* use sfml::graphics::Texture;
use sfml::SfBox;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
pub enum TextureIdentifiers {
    Tile,
}

pub struct TextureManager {
    texture_map: HashMap<TextureIdentifiers, Box<Texture>>,
}

impl TextureManager {
    pub fn new() -> Self {
        TextureManager {
            texture_map: HashMap::new(),
        }
    }
    fn load(&mut self, identifier: TextureIdentifiers, filename: &str) {
        let texture = Texture::from_file(filename).unwrap();
        self.texture_map.insert(identifier, Box::new(texture));
    }
    pub fn from(file_infos: Vec<(TextureIdentifiers, &str)>) -> TextureManager {
        let mut resources = TextureManager::new();
        for file_info in file_infos {
            resources.load(file_info.0, file_info.1);
        }
        resources
    }
    pub fn get(&self, identifier: TextureIdentifiers) -> &SfBox<Texture> {
        match self.texture_map.get(&identifier) {
            Some(resource) => resource,
            None => panic!("Tried to access nonexistant index in resource map"),
        }
    }
}

impl Default for TextureManager {
    fn default() -> Self {
        TextureManager::new()
    }
}

//TODO
// * Add proper error hadling for the resource loading
 */
