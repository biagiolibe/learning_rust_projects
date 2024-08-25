use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(MovingRandomly)]
pub fn random_move(
    ecs: &mut SubWorld,
    #[resource] map: &Map,
) {
    let mut movers = <(&mut Point, &MovingRandomly)>::query();
    movers.for_each_mut(ecs, |(pos, _)| {
        let mut rnd = RandomNumberGenerator::new();
        let destination = *pos + match rnd.range(0, 4) {
            0 => Point::new(-1, 0),
            1 => Point::new(1, 0),
            2 => Point::new(0, -1),
            _ => Point::new(0, 1),
        };
        if map.can_enter_tile(destination){
            *pos = destination;
        }
    })
}