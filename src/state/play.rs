use ggez::{Context, GameResult};
use ggez::event::EventHandler;
use ggez::graphics::{self, Color};
use std::fs;
use std::path::Path;
use ggez::graphics::Image;

pub struct PlayState {
    hero_character: Image,
    shield_ability_position: (f32, f32),
    shiled_ability_cooldown_position: (f32, f32),
    teleport_ability_position: (f32, f32),
    teleport_ability_cooldown_position: (f32, f32),
    ultimate_ability_position: (f32, f32),
    ultimate_ability_cooldown_position: (f32, f32),
}

impl PlayState {
    pub fn new(ctx: &mut Context) -> GameResult<PlayState> {
        let hero_char_path = "/home/pranil/rustProjects/demon_war/resources/hero.png";
        let hero_character = load_image(ctx, hero_char_path);

        let shield_ability_position= (11.0, 11.0);
        let shiled_ability_cooldown_position = (18.0, 18.0);
        let teleport_ability_position= (31.0, 31.0);
        let teleport_ability_cooldown_position= (41.0, 41.0);
        let ultimate_ability_position= (51.0, 51.0);
        let ultimate_ability_cooldown_position= (61.0, 61.0);

        Ok(PlayState{
            hero_character,
            shield_ability_position, 
            shiled_ability_cooldown_position, 
            teleport_ability_cooldown_position, 
            teleport_ability_position, 
            ultimate_ability_cooldown_position, 
            ultimate_ability_position
        })
    }
}

impl EventHandler <ggez::GameError> for PlayState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::from_rgb(0, 0, 0));
        graphics::draw(ctx, &self.hero_character, (ggez::mint::Point2 {x: 0.0, y: 0.0},))?;
        graphics::present(ctx)?;
        Ok(())
    }
}

fn load_image(ctx: &mut Context, file_path: &str) -> graphics::Image {
    let image_bytes = fs::read(file_path).unwrap();
    let image = graphics::Image::from_bytes(ctx, &image_bytes).unwrap();
    image
}



