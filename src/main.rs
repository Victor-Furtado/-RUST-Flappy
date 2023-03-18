use bracket_lib::prelude::*;

enum GameMode {
    Menu,
    Play,
    Over,
}

struct State {
    mode: GameMode,
}

impl State {
    fn new() -> Self {
        State {
            mode: GameMode::Menu,
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        self.mode = GameMode::Over;
    }
    fn menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(10, "Welcome to Flappy Dragon");
        ctx.print_centered(15, "(P) Play Game");
        ctx.print_centered(17, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(10, "You are dead!");
        ctx.print_centered(15, "(P) Play Again");
        ctx.print_centered(17, "(Q) Quit Game");
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn restart(&mut self) {
        self.mode = GameMode::Play;
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.menu(ctx),
            GameMode::Over => self.dead(ctx),
            GameMode::Play => self.play(ctx),
            _ => return,
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50().with_title("Flappy").build()?;

    main_loop(context, State::new())
}
