use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(FieldOfView)]
pub fn map_render(
    ecs: &SubWorld,
    #[resource] map: &Map,
    #[resource] camera: &Camera,
    #[resource] theme: &Box<dyn MapTheme>,
) {
    let mut field_of_view = <&FieldOfView>::query().filter(component::<Player>());
    let player_fov = field_of_view.iter(ecs).next().unwrap();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);
    for y in camera.top_y..=camera.bottom_y {
        for x in camera.left_x..camera.right_x {
            let point = Point::new(x, y);
            let offset = Point::new(camera.left_x, camera.top_y);
            let idx = map_idx(point.x, point.y);
            if map.in_bounds(point) && (player_fov.visible_tiles.contains(&point) || map.revealed_tiles[idx]) {
                let idx = map_idx(x, y);
                let glyph = theme.tile_to_render(map.tiles[idx]);
                let tint = if player_fov.visible_tiles.contains(&point) {
                    WHITE
                } else {
                    DARK_GRAY
                };
                draw_batch.set(
                    point - offset,
                    ColorPair::new(tint, BLACK),
                    glyph,
                );
            }
        }
    }
    draw_batch.submit(0).expect("Map rendering error")
}
