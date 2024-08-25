use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
pub fn entity_render(
    ecs: &SubWorld,
    #[resource] camera: &Camera,
) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);

    let offset = Point::new(camera.left_x, camera.top_y);
    <(&Point, &Render)>::query().for_each(ecs, |(pos, render)| {
        draw_batch.set(
            *pos - offset,
            render.color,
            render.glyph,
        );
    });
    draw_batch.submit(5000).expect("Entities rendering error")
}
