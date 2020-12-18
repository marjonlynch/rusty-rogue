mod map;
mod map_builder;
mod player;
pub mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::map::*;
    pub use crate::player::*;
    pub use crate::map_builder::*;
}

use prelude::*;

enum GameMode {
    Menu,
    Playing,
    End,
}

const FRAME_DURATION : f32 = 75.0;

struct State {
    map: Map,
    player: Player,
    frame_time: f32,
    score: i32, 
    mode: GameMode,
}

impl State {
    fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::build(&mut rng);

        State {
            map: map_builder.map,
            player: Player::new(map_builder.player_start),
            score: 0,
            frame_time: 0.0,
            mode: GameMode::Playing,
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
        }
        self.player.render(ctx);
       // self.mode = GameMode::End;
        ctx.print(0, 1, &format!("Score {}", self.score));
    }

    fn restart(&mut self) {
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::build(&mut rng);
        self.player = Player::new(map_builder.player_start);
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
        self.score = 0;
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You are dead!");
        ctx.print_centered(6, &format!("You earned {} points", self.score));
        ctx.print_centered(8, "(P) Play Again");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to Rusty Rogue");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
}

// implement the trait GameState for struct State
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::End => self.dead(ctx),
            GameMode::Playing => {
                ctx.cls();
                self.player.update(ctx, &self.map);
                self.map.render(ctx);
                self.player.render(ctx);
            }
        }
        // ctx.cls();
        // ctx.print(1, 1, "Hello, Bracket Terminal!");
        // ctx.draw_box(5, 5, 20, 20, WHITE_SMOKE, RED2);
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;

    main_loop(context, State::new())
}
