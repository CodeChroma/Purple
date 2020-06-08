use amethyst::ecs::Entity;

use serde::{Deserialize, Serialize};

use tiled::{parse_with_path, Map};

use std::{fs::File, io::BufReader, path::Path};

pub struct Level {
    pub map: Map,
    pub width: usize,
    pub tiles: Vec<Entity>,
}

impl Level {
    pub fn new(level_file: String) -> Self {
        let file = File::open(&Path::new(&level_file)).unwrap();
        let reader = BufReader::new(file);
        let map = parse_with_path(reader, &Path::new("assets/tilemaps/1-1tileset.tsx")).unwrap();

        Self {
            width: map.layers[0].tiles[0].len() - 1,
            map,
            tiles: Vec::new(),
        }
    }
}

impl Default for Level {
    fn default() -> Self {
        Level::new("assets/tilemaps/1-1.tmx".to_string())
    }
}
