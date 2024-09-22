use crate::prelude::*;

#[system]
#[read_component(WantsActivateItem)]
#[read_component(ProvidesHealing)]
#[read_component(ProvidesDungeonMap)]
#[read_component(Player)]
#[write_component(Health)]
pub fn use_items(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] map: &mut Map,
) {
    let mut healing_to_apply: Vec<(Entity, i32)> = Vec::new();
    let mut items_to_activate = <(Entity, &WantsActivateItem)>::query();
    items_to_activate
        .for_each(ecs, |(message, wants_to_activate)| {
            let item = ecs.entry_ref(wants_to_activate.item);
            if let Ok(item) = item {
                if let Ok(healing) = item.get_component::<ProvidesHealing>() {
                    healing_to_apply.push((wants_to_activate.used_by, healing.amount));
                }
                if let Ok(_) = item.get_component::<ProvidesDungeonMap>() {
                    map.revealed_tiles.iter_mut().for_each(|revealed| *revealed = true);
                }
                commands.remove(*message);
                commands.remove(wants_to_activate.item);
            }
        });
    for heal in healing_to_apply {
        if let Ok(mut to_be_healed) = ecs.entry_mut(heal.0) {
            if let Ok(health) = to_be_healed.get_component_mut::<Health>() {
                health.current = i32::min(health.max, health.current + heal.1);
            }
        }
    }
}