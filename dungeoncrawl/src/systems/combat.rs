use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[read_component(Player)]
#[write_component(Health)]
#[read_component(Carried)]
#[read_component(Damage)]
pub fn combat(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    let mut attackers = <(Entity, &WantsToAttack)>::query();
    let victims = attackers
        .iter(ecs)
        .map(|(entity, wants_to_attack)| (*entity, wants_to_attack.attacker, wants_to_attack.victim))
        .collect::<Vec<(Entity, Entity, Entity)>>();
    victims.iter().for_each(|(message,attacker,  victim)| {
        let base_damage = if let Ok(attacker) = ecs.entry_ref(*attacker){
            if let Ok(damage) = attacker.get_component::<Damage>(){
                damage.0
            } else {
                0
            }
        } else {
            0
        };
        let weapon_damage: i32 = <(&Carried, &Damage)>::query().iter(ecs)
            .filter(|(carried, _)| carried.0 == *attacker)
            .map(|(_, damage)| damage.0)
            .sum();
        let total_damage = base_damage + weapon_damage;

        let is_player = ecs.entry_ref(*victim).unwrap().get_component::<Player>().is_ok();
        if let Ok(health) = ecs.entry_mut(*victim).unwrap().get_component_mut::<Health>() {
            health.current -= total_damage;
            if health.current < 1 && !is_player {
                commands.remove(*victim);
            }
        }
        commands.remove(*message);
    });
}