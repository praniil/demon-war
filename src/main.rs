mod state;
use ggez::{conf, ContextBuilder, GameResult};
use ggez::event;
use state::play::PlayState;


fn main() -> GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new("demon-war", "Pranil Parajuli").window_setup(conf::WindowSetup::default().title("demon war")).window_mode(conf::WindowMode::default().fullscreen_type(conf::FullscreenType::True)).build()?;   

    ggez::graphics::set_drawable_size(&mut ctx, 1920., 1080.)?;

    // let mut state = game::
    let initial_state = PlayState::new(&mut ctx).unwrap();
    event::run(ctx, event_loop, initial_state);
}
