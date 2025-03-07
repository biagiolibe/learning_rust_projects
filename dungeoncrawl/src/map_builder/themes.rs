use crate::map_builder::MapTheme;
use crate::prelude::*;

pub struct DungeonTheme {}

impl DungeonTheme{
    pub fn new() -> Box<dyn MapTheme>{
        Box::new(Self{})
    }
}

impl MapTheme for DungeonTheme{
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
        match tile_type {
            TileType::Floor => to_cp437('.'),
            TileType::Wall => to_cp437('#'),
            TileType::Exit => to_cp437('>')
        }
    }
    fn who_am_i(&self) -> String {
        "DungeonTheme".to_string()
    }
}

pub struct ForestTheme {}

impl ForestTheme {
    pub fn new() -> Box<dyn MapTheme>{
        Box::new(Self{})
    }
}

impl MapTheme for ForestTheme {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
        match tile_type {
            TileType::Floor => to_cp437(';'),
            TileType::Wall => to_cp437('"'),
            TileType::Exit => to_cp437('>')
        }
    }
    fn who_am_i(&self) -> String {
        "ForestTheme".to_string()
    }
}