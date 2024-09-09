use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[write_component(Health)]
pub fn combat(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    let mut attackers = <(Entity, &WantsToAttack)>::query();
    let victims = attackers
        .iter(ecs)
        .map(|(entity, wants_to_attack)| (*entity, wants_to_attack.victim))
        .collect::<Vec<(Entity, Entity)>>();
    victims.iter().for_each(|(attacker_entity, victim)| {
        if let Ok(health) = ecs.entry_mut(*victim).unwrap().get_component_mut::<Health>() {
            health.current -= 1;
            if health.current <1 {
                commands.remove(*victim);
            }
        }
        commands.remove(*attacker_entity);
    });
}