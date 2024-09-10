use crate::prelude::*;

#[system]
#[read_component(ChasingPlayer)]
#[read_component(Point)]
#[read_component(Health)]
#[read_component(Player)]
pub fn chasing(
    ecs: &SubWorld,
    #[resource] map: &Map,
    commands: &mut CommandBuffer,
) {
    let mut movers = <(Entity, &Point, &ChasingPlayer)>::query();
    let mut who_has_health = <(Entity, &Point, &Health)>::query();
    let mut player = <(&Point, &Player)>::query();

    let player_position = player.iter(ecs).nth(0).unwrap().0;
    let player_position_idx = map_idx(player_position.x, player_position.y);

    let search_targets = vec![player_position_idx];
    let dijkstra_map = DijkstraMap::new(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        &search_targets,
        map,
        1024.0);
    movers.for_each(ecs, |(entity, pos, _)| {
        let idx = map_idx(pos.x, pos.y);
        if let Some(destination) = DijkstraMap::find_lowest_exit(&dijkstra_map, idx, map) {
            let distance = DistanceAlg::Pythagoras.distance2d(*pos, *player_position);
            let destination = if distance > 1.2 {
                map.index_to_point2d(destination)
            } else {
                *player_position
            };

            let mut attacked = false;
            who_has_health
                .iter(ecs)
                .filter(|(_, pos, _)| **pos == destination)
                .for_each(|(victim, _, _)| {
                    if ecs.entry_ref(*victim).unwrap().get_component::<Player>().is_ok() {
                        commands.push(((), WantsToAttack {
                            attacker: *entity,
                            victim: *victim,
                        }));
                        attacked = true;
                    }
                });
            if !attacked {
                commands.push(((), WantsToMove {
                    entity: *entity,
                    destination,
                }));
            }
        }
    });
}