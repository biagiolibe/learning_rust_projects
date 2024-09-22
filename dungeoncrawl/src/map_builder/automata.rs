use crate::map::TileType::{Floor, Wall};
use crate::map_builder::themes::DungeonTheme;
use crate::map_builder::MapArchitect;
use crate::prelude::*;
pub struct CellularAutomataArchitect {}

impl CellularAutomataArchitect {
    fn random_noise_map(&mut self, random: &mut RandomNumberGenerator, map: &mut Map) {
        map.tiles.iter_mut().for_each(|tile_type| {
            if let 0..55 = random.range(0, 100) {
                *tile_type = TileType::Wall;
            } else { *tile_type = TileType::Floor; }
        });
    }

    fn count_neighbors(&mut self, x: i32, y: i32, map: &Map) -> usize {
        let mut neighbors = 0;
        for ix in -1..=1 {
            for iy in -1..=1 {
                if !(ix == 0 && iy == 0) &&
                    map.tiles[map_idx(x + ix, y + iy)] == Wall {
                    neighbors += 1;
                }
            }
        }
        neighbors
    }

    fn iteration(&mut self, map: &mut Map) {
        let mut tiles: Vec<TileType> = map.tiles.clone();
        for x in 1..SCREEN_WIDTH - 1 {
            for y in 1..SCREEN_HEIGHT - 1 {
                let neighbors = self.count_neighbors(x, y, map);
                if neighbors == 0 || neighbors > 4 {
                    tiles[map_idx(x, y)] = Wall;
                } else {
                    tiles[map_idx(x, y)] = Floor;
                }
            }
        }
        map.tiles = tiles;
    }

    fn find_start(&mut self, map: &Map) -> Point{
        let map_center = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        let closest_point = map.tiles.iter()
            .enumerate()
            .filter(|(_, tile_type)| **tile_type == Floor)
            .map(|(idx, _)| (idx, DistanceAlg::Pythagoras.distance2d(map.index_to_point2d(idx), map_center)))
            .min_by(|(_, distance_a), (_, distance_b)| distance_a.partial_cmp(distance_b).unwrap())
            .map(|(idx, _)| idx)
            .unwrap();
        map.index_to_point2d(closest_point)
    }
}

impl MapArchitect for CellularAutomataArchitect {
    fn new(&mut self, random: &mut RandomNumberGenerator) -> MapBuilder {
        let mut map_builder = MapBuilder {
            map: Map::new(),
            theme: DungeonTheme::new(),
            rooms: Vec::new(),
            monster_spawns: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
        };
        self.random_noise_map(random, &mut map_builder.map);
        for _ in 0..10{
            self.iteration(&mut map_builder.map);
        }
        let start = self.find_start(&map_builder.map);
        map_builder.player_start = start;
        map_builder.monster_spawns = map_builder.spawn_monsters(&start, random);
        map_builder.amulet_start = map_builder.find_most_distant();
        map_builder
    }

    fn who_am_i(&self) -> String{
        "CellularAutomataArchitect".to_string()
    }
}