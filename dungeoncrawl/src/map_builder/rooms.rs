use crate::map_builder::MapArchitect;
use crate::map_builder::themes::DungeonTheme;
use crate::prelude::*;
pub struct RoomsArchitect {}

impl RoomsArchitect{}

impl MapArchitect for RoomsArchitect {
    fn new(&mut self, random: &mut RandomNumberGenerator) -> MapBuilder {
        let mut map_builder = MapBuilder {
            map: Map::new(),
            theme: DungeonTheme::new(),
            rooms: Vec::new(),
            monster_spawns: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
        };
        map_builder.fill(TileType::Wall);
        map_builder.build_random_rooms(random);
        map_builder.build_corridors(random);
        map_builder.player_start = map_builder.rooms
            .first()
            .map_or(Point::zero(), |r| r.center());
        map_builder.amulet_start = map_builder.find_most_distant();
        map_builder.rooms.iter()
            .skip(1)
            .for_each(|room| map_builder.monster_spawns.push(room.center()));
        map_builder
    }

    fn who_am_i(&self) -> String {
        "RoomsArchitect".to_string()
    }
}