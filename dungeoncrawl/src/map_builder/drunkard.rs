use crate::map::TileType::{Floor, Wall};
use crate::map_builder::MapArchitect;
use crate::map_builder::themes::DungeonTheme;
use crate::prelude::*;

const STAGGER_DISTANCE: usize = 400;
const DESIRED_FLOOR: usize = ((SCREEN_WIDTH * SCREEN_HEIGHT) as usize) / 3;
pub struct DrunkardsWalkArchitect {}

impl DrunkardsWalkArchitect {
    fn drunkard(&mut self,
                start: &Point,
                random: &mut RandomNumberGenerator,
                map: &mut Map) {
        let mut drunkard_position = start.clone();
        let mut distance_staggered = 0;
        loop {
            let drunkard_position_index = map.point2d_to_index(drunkard_position);
            map.tiles[drunkard_position_index] = Floor;
            match random.range(0, 4) {
                0 => drunkard_position.x -= 1,
                1 => drunkard_position.x += 1,
                2 => drunkard_position.y += 1,
                _ => drunkard_position.y -= 1,
            }
            if !map.in_bounds(drunkard_position) {
                break;
            }
            distance_staggered += 1;
            if distance_staggered > STAGGER_DISTANCE {
                break;
            }
        }
    }
}

impl MapArchitect for DrunkardsWalkArchitect {
    fn new(&mut self, random: &mut RandomNumberGenerator) -> MapBuilder {
        let mut map_builder = MapBuilder {
            map: Map::new(),
            theme: DungeonTheme::new(),
            rooms: Vec::new(),
            monster_spawns: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
        };
        map_builder.fill(Wall);
        let center_position = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        self.drunkard(&center_position, random, &mut map_builder.map);
        while map_builder.map.tiles.iter()
            .filter(|tile_type| **tile_type == Floor)
            .count() < DESIRED_FLOOR {
            let random_starting = Point::new(random.range(0, SCREEN_WIDTH), random.range(0, SCREEN_HEIGHT));
            self.drunkard(&random_starting, random, &mut map_builder.map);
            let dijkstra_map = DijkstraMap::new(
                SCREEN_WIDTH,
                SCREEN_HEIGHT,
                &vec![map_builder.map.point2d_to_index(center_position)],
                &map_builder.map,
                1024.0);
            dijkstra_map.map.iter()
                .enumerate()
                .filter(|(_, distance)| **distance > 2000.0)
                .for_each(|(index, _)| map_builder.map.tiles[index] = Wall);
        }
        map_builder.monster_spawns = map_builder.spawn_monsters(&center_position, random);
        map_builder.player_start = center_position;
        map_builder.amulet_start = map_builder.find_most_distant();
        map_builder
    }

    fn who_am_i(&self) -> String{
        "DrunkardsWalkArchitect".to_string()
    }
}