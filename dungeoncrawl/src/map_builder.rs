use crate::prelude::*;

const NUM_ROOMS: usize = 20;

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub player_start: Point,
    pub amulet_start: Point,
}

impl MapBuilder {
    pub fn new(random: &mut RandomNumberGenerator) -> Self {
        let mut map_builder = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
        };
        map_builder.fill(TileType::Wall);
        map_builder.build_random_rooms(random);
        map_builder.build_corridors(random);
        map_builder.player_start = map_builder.rooms.first().map_or(Point::zero(), |r| r.center());
        let dijkstra_map = DijkstraMap::new(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            &vec![map_builder.map.point2d_to_index(map_builder.player_start)],
            &map_builder.map,
            1024.0,
        );
        const UNREACHABLE: f32 = f32::MAX;
        let max_distance_idx = dijkstra_map.map
            .iter()
            .enumerate()
            .filter(|(_idx, dist)| **dist < UNREACHABLE)
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap().0;
        map_builder.amulet_start = map_builder.map.index_to_point2d(max_distance_idx);
        map_builder
    }

    fn fill(&mut self, tile_type: TileType) {
        self.map.tiles.iter_mut().for_each(|tile| *tile = tile_type);
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
        use std::cmp::{min, max};
        for y in min(y1, y2)..max(y1, y2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn create_horizontal_tunnel(&mut self, y: i32, x1: i32, x2: i32) {
        use std::cmp::{min, max};
        for x in min(x1, x2)..max(x1, x2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx] = TileType::Floor;
            }
        }
    }
}
