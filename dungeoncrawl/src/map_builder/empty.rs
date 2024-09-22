use crate::map::TileType::Floor;
use crate::map_builder::MapArchitect;
use crate::map_builder::themes::DungeonTheme;
use crate::prelude::*;
pub struct EmptyArchitect {}

impl MapArchitect for EmptyArchitect {
    fn new(&mut self, random: &mut RandomNumberGenerator) -> MapBuilder {
        let mut map_builder = MapBuilder {
            map: Map::new(),
            theme: DungeonTheme::new(),
            rooms: Vec::new(),
            monster_spawns: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
        };
        map_builder.fill(Floor);
        map_builder.player_start = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        map_builder.amulet_start = map_builder.find_most_distant();
        for _ in 0..50 {
            map_builder.monster_spawns.push(
                Point::new(random.range(1, SCREEN_WIDTH), random.range(1, SCREEN_HEIGHT)),
            )
        }
        map_builder
    }

    fn who_am_i(&self) -> String {
        "EmptyArchitect".to_string()
    }
}