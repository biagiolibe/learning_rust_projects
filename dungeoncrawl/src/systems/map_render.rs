use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(FieldOfView)]
pub fn map_render(
    ecs: &SubWorld,
    #[resource] map: &Map,
    #[resource] camera: &Camera,
) {
    let mut field_of_view = <&FieldOfView>::query().filter(component::<Player>());
    let player_fov = field_of_view.iter(ecs).next().unwrap();
    
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);
    for y in camera.top_y..=camera.bottom_y {
        for x in camera.left_x..camera.right_x {
            let point = Point::new(x, y);
            let offset = Point::new(camera.left_x, camera.top_y);
            if map.in_bounds(point) && player_fov.visible_tiles.contains(&point) {
                let idx = map_idx(x, y);
                let glyph = match map.tiles[idx] {
                    TileType::Floor => to_cp437('.'),
                    TileType::Wall => to_cp437('#'),
                };
                draw_batch.set(
                    point - offset,
                    ColorPair::new(WHITE, BLACK),
                    glyph,
                );
            }
        }
    }
    draw_batch.submit(0).expect("Map rendering error")
}
