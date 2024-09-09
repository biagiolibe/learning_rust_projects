use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(MovingRandomly)]
#[read_component(Health)]
#[read_component(Player)]
pub fn random_move(
    ecs: &SubWorld,
    commands: &mut CommandBuffer,
) {
    let mut movers = <(Entity, &Point, &MovingRandomly)>::query();
    let mut who_has_health = <(Entity, &Point, &Health)>::query();

    movers.for_each(ecs, |(entity, pos, _)| {
        let mut rnd = RandomNumberGenerator::new();
        let delta = match rnd.range(0, 4) {
            0 => Point::new(-1, 0),
            1 => Point::new(1, 0),
            2 => Point::new(0, -1),
            _ => Point::new(0, 1),
        };
        let destination = *pos + delta;
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
    });
}