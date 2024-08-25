use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
pub fn collisions(
    ecs: &SubWorld,
    commands: &mut CommandBuffer,
) {
    let mut player_positions: Vec<Point> = Vec::new();
    <&Point>::query().filter(component::<Player>()).for_each(ecs, |pos| player_positions.push(*pos));
    let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());
    enemies.iter(ecs)
        .filter(|(_,pos)| player_positions.contains(*pos))
        .for_each(|(entity, _)| {
        commands.remove(*entity);
    });
}
