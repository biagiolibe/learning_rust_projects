use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(AmuletOfYala)]
#[read_component(Point)]
pub fn end_turn(
    ecs: &SubWorld,
    #[resource] turn_state: &mut TurnState,
    #[resource] map: &Map,
) {
    let mut player = <(&Health, &Point)>::query().filter(component::<Player>());
    let mut amulet = <&Point>::query().filter(component::<AmuletOfYala>());
    
    let mut new_state = match turn_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        _ => turn_state.clone(),
    };
    let amulet_default_pos = Point::new(-1, -1);
    let amulet_position = amulet.iter(ecs).nth(0).unwrap_or(&amulet_default_pos);

    player.for_each(ecs, |(hp, pos)| {
        if hp.current < 1 {
            new_state = TurnState::GameOver;
        }
        if pos == amulet_position {
            new_state = TurnState::Victory;
        }
        if map.tiles[map.point2d_to_index(*pos)] == TileType::Exit {
            new_state = TurnState::NextLevel;
        }
    });
    *turn_state = new_state;
}