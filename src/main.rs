use bracket_lib::prelude::*;

enum GameMode {
    Menu,
    Playing,
    End,
}

// const SCREEN_WIDTH : i32 = 80;
// const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION : f32 = 75.0;

struct State {
    player: Player,
    frame_time: f32,
    score: i32, 
    mode: GameMode,
}
struct Player {
    x: i32,
    y: i32,
    velocity: f32,
}

impl State {
    fn new() -> Self {
        State {
            player: Player::new(5, 25),
            score: 0,
            frame_time: 0.0,
            mode: GameMode::Menu,
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
        }
        self.player.render(ctx);
        self.mode = GameMode::End;
        ctx.print(0, 1, &format!("Score {}", self.score));
    }

    fn restart(&mut self) {
        self.player = Player::new(5, 25);
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

impl Player {
    fn new(x: i32, y: i32) -> Self {
        Player {
            x,
            y,
            velocity: 0.0,
        }
    }

    // also can use RGB::from_u8() or RGB::from_hex() to get color
    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(
            0,
            self.y,
            YELLOW,
            BLACK,
            to_cp437('@')
        );
    }
}

// implement the trait GameState for struct State
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::End => self.dead(ctx),
            GameMode::Playing => self.play(ctx),
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
