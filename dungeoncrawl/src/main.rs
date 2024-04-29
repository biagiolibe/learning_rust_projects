mod map;
mod player;

mod prelude{
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::map::*;
    pub use crate::player::*;
}

use prelude::*;

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        .build()?;
    main_loop(context, State::new())
}

struct State {
    map: Map,
    player: Player,
}

impl State {
    fn new() -> Self {
        Self { map: Map::new(), player: Player::new(Point { x: SCREEN_WIDTH/2, y: SCREEN_HEIGHT/2 }) }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.player.move_in_map(ctx, &self.map);
        self.map.render(ctx);
        self.player.render(ctx);
    }
}
