mod state;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::event;
use state::play::{self, PlayState};


fn main() -> GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new("demon-war", "Pranil Parajuli").build()?;

    // let mut state = game::
    let initial_state = PlayState::new(&mut ctx).unwrap();
    event::run(ctx, event_loop, initial_state);
}
