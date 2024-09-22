use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Name)]
#[read_component(Item)]
#[read_component(Carried)]
pub fn hud(
    ecs: &SubWorld
) {
    let mut player_health_query = <(Entity, &Health)>::query().filter(component::<Player>());
    let (player, player_health) = player_health_query.iter(ecs).nth(0).unwrap();
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    draw_batch.print_centered(1, "Explore the Dungeon. Cursor keys to move.");
    draw_batch.bar_horizontal(Point::zero(),
                              SCREEN_WIDTH * 2,
                              player_health.current,
                              player_health.max,
                              ColorPair::new(RED, BLACK));
    draw_batch.print_color_centered(0,
                                    format!(" Health: {} / {}", player_health.current, player_health.max),
                                    ColorPair::new(WHITE, RED));
    let mut items = <(&Item, &Name, &Carried)>::query();
    let mut carried_items = 0;
    items.iter(ecs)
        .filter(|(_item, _name, carried)| carried.0 == *player)
        .for_each(|(_item, name, _carried)| {
            draw_batch.print(
                Point::new(3, 3+carried_items),
                format!("{}: {}", carried_items + 1, name.0));
            carried_items += 1;
        });
    if carried_items > 0 {
        draw_batch.print_color(
            Point::new(3, 2),
            "Items carried",
            ColorPair::new(YELLOW, BLACK));
    }
    draw_batch.submit(10000).expect("HUD rendering error")
}
