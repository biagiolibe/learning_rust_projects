use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
#[write_component(Health)]
#[read_component(Item)]
#[read_component(Carried)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            VirtualKeyCode::G => {
                let (player, player_position) = players.iter(ecs)
                    .find_map(|(entity, position)| Some((*entity, *position)))
                    .unwrap();
                let mut items = <(Entity, &Item, &Point)>::query();
                items.iter(ecs)
                    .filter(|(_entity, _item, item_position)| player_position == **item_position)
                    .for_each(|(entity, _item, _item_position)| {
                        commands.remove_component::<Point>(*entity);
                        commands.add_component(*entity, Carried(player));
                    });
                Point::new(0, 0)
            }
            VirtualKeyCode::Key1 => use_item(0, ecs, commands),
            VirtualKeyCode::Key2 => use_item(1, ecs, commands),
            VirtualKeyCode::Key3 => use_item(2, ecs, commands),
            VirtualKeyCode::Key4 => use_item(3, ecs, commands),
            VirtualKeyCode::Key5 => use_item(4, ecs, commands),
            VirtualKeyCode::Key6 => use_item(5, ecs, commands),
            VirtualKeyCode::Key7 => use_item(6, ecs, commands),
            VirtualKeyCode::Key8 => use_item(7, ecs, commands),
            VirtualKeyCode::Key9 => use_item(8, ecs, commands),
            _ => Point::new(0, 0),
        };
        let (player_entity, destination) = players
            .iter(ecs)
            .find_map(|(entity, pos)| Some((*entity, *pos + delta)))
            .unwrap();
        let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());
        if delta.x != 0 || delta.y != 0 {
            let mut hit_something = false;
            enemies.iter(ecs)
                .filter(|(_, pos)| **pos == destination)
                .for_each(|(entity, _)| {
                    hit_something = true;
                    commands.push(((), WantsToAttack {
                        attacker: player_entity,
                        victim: *entity,
                    }));
                });
            if !hit_something {
                commands.push(((), WantsToMove {
                    entity: player_entity,
                    destination,
                }));
            }
        }
        *turn_state = TurnState::PlayerTurn;
    }

    fn use_item(n: usize, ecs: &mut SubWorld, commands: &mut CommandBuffer) -> Point {
        let player = <(Entity, &Player)>::query().iter(ecs)
            .find_map(|(entity, _player)| Some(*entity))
            .unwrap();

        let mut items = <(Entity, &Item, &Carried)>::query();
        let item_to_use = items.iter(ecs)
            .filter(|(_entity, _item, carried)| carried.0 == player)
            .enumerate()
            .filter(|(index, (_entity, _item, _carried))| n == *index)
            .find_map(|(_, (entity, _item, _carried))| Some(*entity));
        if let Some(item_entity) = item_to_use {
            commands.push(((), WantsActivateItem {
                used_by: player,
                item: item_entity,
            }));
        }
        Point::zero()
    }
}