use crate::map::TileType::{Floor, Wall};
use crate::prelude::*;

const FORTRESS: (&str, i32, i32) = (
    "
------------
---######---
---#----#---
---#-M--#---
-###----###-
--M------M--
-###----###-
---#----#---
---#----#---
---######---
------------
    ",
    12, 11);

pub fn apply_prefab(map_builder: &mut MapBuilder,
                    random: &mut RandomNumberGenerator) {
    let mut placement = None;
    let dijkstra_map = DijkstraMap::new(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        &vec![map_builder.map.point2d_to_index(map_builder.player_start)],
        &map_builder.map,
        1024.0,
    );
    let mut attempts = 0;
    while attempts < 10 && placement.is_none() {
        let dimensions = Rect::with_size(
            random.range(0, SCREEN_WIDTH - FORTRESS.1),
            random.range(0, SCREEN_HEIGHT - FORTRESS.2),
            FORTRESS.1,
            FORTRESS.2,
        );
        let mut can_place = false;
        dimensions.for_each(|point| {
            let distance_from_player = dijkstra_map.map[map_builder.map.point2d_to_index(point)];
            if distance_from_player < 2000.0 && distance_from_player > 20.0 && map_builder.amulet_start != point {
                can_place = true;
            };
        });
        if can_place {
            placement = Some(Point::new(dimensions.x1, dimensions.y1));
            let points = dimensions.point_set();
            map_builder.monster_spawns.retain(|point| !points.contains(point));
        }
        attempts += 1;
    };
    if let Some(placement) = placement {
        println!("Prefab placeable in ({},{})", placement.x, placement.y);
        let prefab_string_vec: Vec<char> = FORTRESS.0.chars()
            .filter(|c| *c != '\r' && *c != '\n')
            .collect();
        let mut i = 0;
        for y in placement.y..placement.y + FORTRESS.2 {
            for x in placement.x..placement.x + FORTRESS.1 {
                let index = map_idx(x, y);
                let prefab_char = prefab_string_vec[i];
                match prefab_char {
                    '-' => map_builder.map.tiles[index] = Floor,
                    '#' => map_builder.map.tiles[index] = Wall,
                    'M' => {
                        map_builder.map.tiles[index] = Floor;
                        map_builder.monster_spawns.push(Point::new(x, y));
                    }
                    _ => println!("unhandled char in prefab structure (char {})", prefab_char)
                }
                i += 1;
            }
        }
    }
}