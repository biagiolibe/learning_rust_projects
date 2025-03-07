use crate::map::TileType::Exit;
use prelude::*;
use std::collections::HashSet;

mod camera;
mod components;
mod map;
mod map_builder;
mod spawner;
mod systems;
mod turn_state;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;

    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;

    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_font("terminal8x8.png", 8, 8)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(SCREEN_WIDTH * 2, SCREEN_HEIGHT * 2, "terminal8x8.png")
        .build()?;
    main_loop(context, State::new())
}

struct State {
    ecs: World,
    resources: Resources,
    input_systems: Schedule,
    player_systems: Schedule,
    monster_systems: Schedule,
}

impl State {
    fn new() -> Self {
        let (ecs, resources) = Self::initialize_world();
        Self {
            ecs,
            resources,
            input_systems: build_input_scheduler(),
            player_systems: build_player_scheduler(),
            monster_systems: build_monster_scheduler(),
        }
    }

    fn game_over(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(2);
        ctx.print_color_centered(2, RED, BLACK, "Your quest has ended.");
        ctx.print_color_centered(
            4,
            WHITE,
            BLACK,
            "Slain by a monster, your hero's journey has came to a premature end.",
        );
        ctx.print_color_centered(
            8,
            YELLOW,
            BLACK,
            "Don't worry, you can always try again with a new hero.",
        );
        ctx.print_color_centered(9, GREEN, BLACK, "Press 1 to play again or 'q' to quit.");

        self.handle_end_game_input(ctx);
    }

    fn handle_end_game_input(&mut self, ctx: &mut BTerm) {
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Key1 => {
                    let (ecs, resources) = Self::initialize_world();
                    self.ecs = ecs;
                    self.resources = resources;
                }
                VirtualKeyCode::Q => {
                    ctx.quit();
                }
                _ => {}
            }
        }
    }

    fn advance_level(&mut self) {
        let player = <Entity>::query()
            .filter(component::<Player>())
            .iter(&self.ecs)
            .nth(0)
            .unwrap();
        let mut entities_to_keep = HashSet::new();
        entities_to_keep.insert(player);
        <(Entity, &Carried)>::query()
            .iter(&self.ecs)
            .filter(|(_entity, carried)| carried.0 == *player)
            .for_each(|(entity, _carried)| {
                entities_to_keep.insert(entity);
            });
        let mut commands = CommandBuffer::new(&self.ecs);
        Entity::query().for_each(&self.ecs, |entity| {
            if !entities_to_keep.contains(entity) {
                commands.remove(*entity);
            }
        });
        commands.flush(&mut self.ecs, &mut self.resources);
        <&mut FieldOfView>::query().for_each_mut(&mut self.ecs, |fov| {
            fov.is_dirty = true;
        });
        let mut random = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut random);
        let mut map_level = 0;
        <(&mut Player, &mut Point)>::query().for_each_mut(&mut self.ecs, |(player, position)| {
            player.map_level += 1;
            map_level = player.map_level;
            position.x = map_builder.player_start.x;
            position.y = map_builder.player_start.y;
        });
        if map_level == 2 {
            spawn_amulet_of_yala(&mut self.ecs, map_builder.amulet_start);
        } else {
            let exit = map_builder.map.point2d_to_index(map_builder.amulet_start);
            map_builder.map.tiles[exit] = Exit;
        };
        spawn_layer(&mut self.ecs,
                    &mut self.resources,
                    &mut random,
                    map_level as usize,
                    &map_builder.monster_spawns,
        );
        self.resources.insert(map_builder.map);
        self.resources.insert(Camera::new(map_builder.player_start));
        self.resources.insert(TurnState::AwaitingInput);
        self.resources.insert(map_builder.theme);
    }

    fn victory(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(2);
        ctx.print_color_centered(2, GREEN, BLACK, "You have won!");
        ctx.print_color_centered(
            4,
            WHITE,
            BLACK,
            "You put on the Amulet of Yala and feel its power course through your veins.",
        );
        ctx.print_color_centered(
            5,
            WHITE,
            BLACK,
            "Your town is saved, and you can return to your normal life.",
        );
        ctx.print_color_centered(7, GREEN, BLACK, "Press 1 to play again or 'q' to quit.");

        self.handle_end_game_input(ctx);
    }

    fn initialize_world() -> (World, Resources) {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut random = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut random);
        spawn_player(&mut ecs, map_builder.player_start);
        let exit = map_builder.map.point2d_to_index(map_builder.amulet_start);
        map_builder.map.tiles[exit] = Exit;
        //spawn_amulet_of_yala(&mut ecs, map_builder.amulet_start);
        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));
        resources.insert(TurnState::AwaitingInput);
        resources.insert(map_builder.theme);
        spawn_layer(
            &mut ecs,
            &mut resources,
            &mut random,
            0,
            &map_builder.monster_spawns,
        );
        //return world and resources as tuple
        (ecs, resources)
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        ctx.set_active_console(2);
        ctx.cls();
        self.resources.insert(ctx.key);
        self.resources.insert(Point::from_tuple(ctx.mouse_pos()));
        let current_state = self.resources.get::<TurnState>().unwrap().clone();
        match current_state {
            TurnState::AwaitingInput => self
                .input_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::PlayerTurn => self
                .player_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::MonsterTurn => self
                .monster_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::GameOver => self.game_over(ctx),
            TurnState::Victory => self.victory(ctx),
            TurnState::NextLevel => self.advance_level(),
        }
        render_draw_buffer(ctx).expect("Render error");
    }
}
