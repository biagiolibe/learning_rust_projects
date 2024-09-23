use crate::prelude::*;

pub fn spawn_entity(ecs: &mut World, pos: Point, rnd: &mut RandomNumberGenerator) {
    let roll = rnd.roll_dice(1, 6);
    match roll {
        1 => spawn_healing_potion(ecs, pos),
        2 => spawn_magic_mapper(ecs, pos),
        _ => spawn_monster(ecs, pos, rnd)
    };
}

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push(
        (Player{map_level: 0},
         pos,
         Render {
             color: ColorPair::new(WHITE, BLACK),
             glyph: to_cp437('@'),
         },
         Health { current: 10, max: 10 },
         FieldOfView::new(8),)
    );
}

pub fn spawn_monster(ecs: &mut World, pos: Point, rnd: &mut RandomNumberGenerator) {
    let random = rnd.roll_dice(1, 10);
    let (hp, name, glyph) = match random {
        1..=8 => goblin(),
        _ => orc()
    };
    ecs.push(
        (Enemy,
         pos,
         Render {
             color: ColorPair::new(WHITE, BLACK),
             glyph,
         },
         ChasingPlayer,
         Health { current: hp, max: hp },
         Name(name),
         FieldOfView::new(6),
        )
    );
}

pub fn spawn_amulet_of_yala(ecs: &mut World, pos: Point) {
    ecs.push(
        (Item,
         AmuletOfYala,
         pos,
         Render {
             color: ColorPair::new(WHITE, BLACK),
             glyph: to_cp437('|'),
         },
         Name("Amulet of Yala".to_string())
        )
    );
}

pub fn spawn_healing_potion(ecs: &mut World, pos: Point) {
    ecs.push(
        (Item,
         pos,
         Render {
             color: ColorPair::new(WHITE, BLACK),
             glyph: to_cp437('!'),
         },
         Name("Healing Potion".to_string()),
         ProvidesHealing { amount: 6 }
        )
    );
}

pub fn spawn_magic_mapper(ecs: &mut World, pos: Point) {
    ecs.push(
        (Item,
         pos,
         Render {
             color: ColorPair::new(WHITE, BLACK),
             glyph: to_cp437('{'),
         },
         Name("Dungeon map".to_string()),
         ProvidesDungeonMap {}
        )
    );
}

pub fn goblin() -> (i32, String, FontCharType) {
    (1, "Goblin".to_string(), to_cp437('g'))
}

pub fn orc() -> (i32, String, FontCharType) {
    (2, "Orc".to_string(), to_cp437('o'))
}