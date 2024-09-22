use crate::map::TileType::Floor;
use crate::map_builder::automata::CellularAutomataArchitect;
use crate::map_builder::drunkard::DrunkardsWalkArchitect;
use crate::map_builder::prefab::apply_prefab;
use crate::map_builder::rooms::RoomsArchitect;
use crate::map_builder::themes::{DungeonTheme, ForestTheme};
use crate::prelude::*;

mod empty;
mod rooms;
mod automata;
mod drunkard;
mod prefab;
mod themes;

const NUM_ROOMS: usize = 20;

trait MapArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder;
    fn who_am_i(&self) -> String;
}

pub trait MapTheme: Sync + Send {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType;
    fn who_am_i(&self) -> String;
}

pub struct MapBuilder {
    pub map: Map,
    pub theme: Box<dyn MapTheme>,
    pub rooms: Vec<Rect>,
    pub monster_spawns: Vec<Point>,
    pub player_start: Point,
    pub amulet_start: Point,
}

impl MapBuilder {
    pub fn new(rnd: &mut RandomNumberGenerator) -> Self {
        let mut architect: Box<dyn MapArchitect> = match rnd.range(0, 3) {
            0 => Box::new(DrunkardsWalkArchitect {}),
            1 => Box::new(RoomsArchitect {}),
            _ => Box::new(CellularAutomataArchitect {}),
        };
        let mut map_builder = architect.new(rnd);
        apply_prefab(&mut map_builder, rnd);
        let theme: Box<dyn MapTheme> = match rnd.range(0, 2) {
            0 => DungeonTheme::new(),
            _ => ForestTheme::new(),
        };
        println!("Generating map using architect {} and theme {}", architect.who_am_i(), theme.who_am_i());
        map_builder.theme = theme;
        map_builder
    }

    fn fill(&mut self, tile_type: TileType) {
        self.map.tiles.iter_mut().for_each(|tile| *tile = tile_type);
    }

    fn spawn_monsters(&mut self, start: &Point, random: &mut RandomNumberGenerator) -> Vec<Point> {
        const NUM_MONSTERS: usize = 50;
        let mut spawnable_points: Vec<Point> = self.map.tiles.iter()
            .enumerate()
            .filter(|(idx, tile_type)| **tile_type == Floor &&
                DistanceAlg::Pythagoras.distance2d(self.map.index_to_point2d(*idx), *start) > 10.0)
            .map(|(idx, _)| self.map.index_to_point2d(idx))
            .collect();
        let mut spawn_points: Vec<Point> = Vec::new();
        for _ in 0..=NUM_MONSTERS {
            let random_spawnable_point = random.random_slice_index(&spawnable_points).unwrap();
            spawn_points.push(spawnable_points[random_spawnable_point].clone());
            spawnable_points.remove(random_spawnable_point);
        }
        spawn_points
    }

    fn find_most_distant(&self) -> Point {
        let dijkstra_map = DijkstraMap::new(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            &vec![self.map.point2d_to_index(self.player_start)],
            &self.map,
            1024.0,
        );
        const UNREACHABLE: f32 = f32::MAX;
        self.map.index_to_point2d(
            dijkstra_map.map
                .iter()
                .enumerate()
                .filter(|(_idx, dist)| **dist < UNREACHABLE)
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                .unwrap().0
        )
    }

    fn build_random_rooms(&mut self, random: &mut RandomNumberGenerator) {
        while self.rooms.len() < NUM_ROOMS {
            let room = Rect::with_size(
                random.range(1, SCREEN_WIDTH - 10),
                random.range(1, SCREEN_HEIGHT - 10),
                random.range(2, 10),
                random.range(2, 10),
            );
            let mut overlap = false;
            for r in self.rooms.iter() {
                if room.intersect(&r) {
                    overlap = true;
                }
            }
            if !overlap {
                room.for_each(|point| {
                    if self.map.in_bounds(point) {
                        self.map.tiles[map_idx(point.x, point.y)] = TileType::Floor;
                    }
                });
                self.rooms.push(room);
            }
        }
    }

    fn build_corridors(&mut self, random: &mut RandomNumberGenerator) {
        let mut rooms = self.rooms.clone();
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));
        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev_room = rooms[i - 1].center();
            let current_room = room.center();
            if random.range(0, 2) == 1 {
                self.create_horizontal_tunnel(prev_room.y, prev_room.x, current_room.x + 1);
                self.create_vertical_tunnel(current_room.x, prev_room.y, current_room.y);
            } else {
                self.create_vertical_tunnel(prev_room.x, prev_room.y, current_room.y);
                self.create_horizontal_tunnel(current_room.y, prev_room.x, current_room.x);
            }
        }
    }

    fn create_vertical_tunnel(&mut self, x: i32, y1: i32, y2: i32) {
        use std::cmp::{max, min};
        for y in min(y1, y2)..max(y1, y2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn create_horizontal_tunnel(&mut self, y: i32, x1: i32, x2: i32) {
        use std::cmp::{max, min};
        for x in min(x1, x2)..max(x1, x2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx] = TileType::Floor;
            }
        }
    }
}
