use ggez::{Context, GameResult};
use ggez::event::EventHandler;
use ggez::graphics::{self, draw, drawable_size, Color, DrawParam};
use std::fs;
use ggez::graphics::Image;
use ggez::event;

struct Arrow {
    position : (f32, f32),
    ongoing : bool,
}
pub struct PlayState {
    hero_character: Image,
    hero_character_size: (f32, f32),
    hero_character_position: (f32, f32),
    shield_ability_position: (f32, f32),
    shiled_ability_cooldown_position: (f32, f32),
    teleport_ability_position: (f32, f32),
    teleport_ability_cooldown_position: (f32, f32),
    ultimate_ability_position: (f32, f32),
    ultimate_ability_cooldown_position: (f32, f32),
    draw_arrow: bool,
    // arrow_ongoing: bool,
    // arrow_position: (f32, f32),
    arrows: Vec<Arrow>,
}


impl PlayState {
    pub fn new(ctx: &mut Context) -> GameResult<PlayState> {
        let (win_width, win_height) = drawable_size(ctx);
        let hero_character = load_image(ctx, "/home/pranil/rustProjects/demon_war/resources/copy_hero.png");
        let hero_character_size = (95.0, 120.0);
        let hero_character_position = (win_width/ 2.0, win_height - 2.0 * hero_character_size.1);
        let shield_ability_position= (11.0, 11.0);
        let shiled_ability_cooldown_position = (18.0, 18.0);
        let teleport_ability_position= (31.0, 31.0);
        let teleport_ability_cooldown_position= (41.0, 41.0);
        let ultimate_ability_position= (51.0, 51.0);
        let ultimate_ability_cooldown_position= (61.0, 61.0);
        let draw_arrow = false;
        // let arrow_ongoing = false;
        let arrow = Arrow {
            position : (hero_character_position.0 - 47.0 + hero_character_size.0 / 2.0, hero_character_position.1),
            ongoing : false
        };
        // let arrow_position = (hero_character_position.0 - 2.0 + hero_character_size.0 / 2.0, hero_character_position.1 - 44.0);
        let arrows = vec![arrow];

        Ok(PlayState{
            hero_character,
            hero_character_size,
            hero_character_position,
            shield_ability_position, 
            shiled_ability_cooldown_position, 
            teleport_ability_cooldown_position, 
            teleport_ability_position, 
            ultimate_ability_cooldown_position, 
            ultimate_ability_position,
            draw_arrow,
            // arrow_position,
            // arrow_ongoing,
            arrows,
        })
    }
}

impl EventHandler <ggez::GameError> for PlayState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        for arrow in &mut self.arrows {
            if arrow.ongoing == true {
                arrow.position.1 -= 5.5;
                println!("{}", arrow.position.1);
                println!("inside the condition");
                if arrow.position.1 < 0.0 {
                    arrow.ongoing = false; 
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::from_rgb(211, 211, 211));
        let hero_dist_rect = graphics::Rect::new(self.hero_character_position.0, self.hero_character_position.1, self.hero_character_size.0, self.hero_character_size.1);
        let hero_draw_param  = DrawParam::default().dest([hero_dist_rect.x, hero_dist_rect.y]).scale([self.hero_character_size.0 / self.hero_character.width() as f32 , self.hero_character_size.1/ self.hero_character.height() as f32]);
        graphics::draw(ctx, &self.hero_character, hero_draw_param)?;

        for arrow in &self.arrows {
            if arrow.ongoing == true {
                let arrow = graphics::Rect::new(arrow.position.0 - 2.0 + self.hero_character_size.0 / 2.0, arrow.position.1 - 44.0, 2.0, 40.0);
                let arrow_mess = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), arrow, Color::from_rgb(0, 0, 0)).unwrap();
                graphics::draw(ctx, &arrow_mess, DrawParam::default()).unwrap();
            }
        }
        graphics::present(ctx)?;
        Ok(())
    }

    fn mouse_button_down_event(
            &mut self,
            _ctx: &mut Context,
            button: event::MouseButton,
            _x: f32,
            _y: f32,
        ) {
        if button == event::MouseButton::Left {
            println!("left button in mouse clicked");
            let new_arrow = Arrow {
                position: (self.hero_character_position.0 - 47.0 + self.hero_character_size.0 / 2.0, self.hero_character_position.1),
                ongoing: true,
            };
            self.arrows.push(new_arrow);
        }
    }
}

fn load_image(ctx: &mut Context, file_path: &str) -> graphics::Image {
    let image_bytes = fs::read(file_path).unwrap();
    let image = graphics::Image::from_bytes(ctx, &image_bytes).unwrap();
    image
}



