use ggez::mint::Vector2;
use ggez::{Context, GameResult};
use ggez::event::EventHandler;
use ggez::graphics::{self, drawable_size, Color, DrawParam};
use std::{fs, vec};
use ggez::graphics::Image;
use ggez::event;
use ggez::input::keyboard::{KeyCode, KeyMods};

const default_pos_hero: f32 = 840.0;

struct Arrow {
    position : (f32, f32),
    ongoing : bool,
}

struct Hero {
    size: (f32, f32),
    position: (f32, f32),
    velocity: Vector2<f32>,
}

trait Gravity {
    fn gravity(&mut self);
}

impl Gravity for Hero {
    fn gravity(&mut self) {
        self.position.1 += 20.0;
        //840.0 position of hero in y
        if self.position.1 > default_pos_hero {
            self.position.1 = default_pos_hero;
        }
    }
}

fn apply_gravity(items: &mut Vec<&mut dyn Gravity>) {
    for item in items {
        item.gravity();
    }
}

pub struct PlayState {
    hero_character: Image,
    shield_ability_position: (f32, f32),
    shiled_ability_cooldown_position: (f32, f32),
    teleport_ability_position: (f32, f32),
    teleport_ability_cooldown_position: (f32, f32),
    ultimate_ability_position: (f32, f32),
    ultimate_ability_cooldown_position: (f32, f32),
    draw_arrow: bool,
    hero: Hero,
    arrows: Vec<Arrow>,
}


impl PlayState {
    pub fn new(ctx: &mut Context) -> GameResult<PlayState> {
        let (win_width, win_height) = drawable_size(ctx);
        let hero_character = load_image(ctx, "/home/pranil/rustProjects/demon_war/resources/copy_hero.png");
        let hero = Hero {
            size: (95.0, 120.0),
            position: (win_width/ 2.0, win_height - 2.0 * 120.0),
            velocity: Vector2 { x: 0.0, y: 0.0 }
        };
        let shield_ability_position= (11.0, 11.0);
        let shiled_ability_cooldown_position = (18.0, 18.0);
        let teleport_ability_position= (31.0, 31.0);
        let teleport_ability_cooldown_position= (41.0, 41.0);
        let ultimate_ability_position= (51.0, 51.0);
        let ultimate_ability_cooldown_position= (61.0, 61.0);
        let draw_arrow = false;
        let arrow = Arrow {
            position : (hero.position.0 - 47.0 + hero.size.0 / 2.0, hero.position.1),
            ongoing : false
        };
        let arrows = vec![arrow];

        Ok(PlayState{
            hero_character,
            shield_ability_position, 
            shiled_ability_cooldown_position, 
            teleport_ability_cooldown_position, 
            teleport_ability_position, 
            ultimate_ability_cooldown_position, 
            ultimate_ability_position,
            draw_arrow,
            arrows,
            hero,
        })
    }
}

impl EventHandler <ggez::GameError> for PlayState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.hero.velocity = Vector2{x: 0.0, y: 0.0};
        let mut gravity_objects: Vec<&mut dyn Gravity> = vec![&mut self.hero];
        apply_gravity(&mut gravity_objects);
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
        let hero_dist_rect = graphics::Rect::new(self.hero.position.0, self.hero.position.1, self.hero.size.0, self.hero.size.1);
        let hero_draw_param  = DrawParam::default().dest([hero_dist_rect.x, hero_dist_rect.y]).scale([self.hero.size.0 / self.hero_character.width() as f32 , self.hero.size.1/ self.hero_character.height() as f32]);
        graphics::draw(ctx, &self.hero_character, hero_draw_param)?;

        for arrow in &self.arrows {
            if arrow.ongoing == true {
                let arrow = graphics::Rect::new(arrow.position.0 - 2.0 + self.hero.size.0 / 2.0, arrow.position.1 - 44.0, 2.0, 40.0);
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
                position: (self.hero.position.0 - 47.0 + self.hero.size.0 / 2.0, self.hero.position.1),
                ongoing: true,
            };
            self.arrows.push(new_arrow);
        }
    }

    fn key_down_event(
            &mut self,
            _ctx: &mut Context,
            keycode: KeyCode,
            _keymods: KeyMods,
            _repeat: bool,
        ) {
        match keycode {
            KeyCode:: A => {
                self.hero.velocity.x -= 80.0;
                self.hero.position.0 += self.hero.velocity.x;
            }
            KeyCode :: W => {
                self.hero.position.1 -= 200.0;
                println!("pressed w");
            }
            KeyCode::D => {
                self.hero.velocity.x += 80.0;
                self.hero.position.0 += self.hero.velocity.x;
            }
            _ => {}
        }
    }
}

fn load_image(ctx: &mut Context, file_path: &str) -> graphics::Image {
    let image_bytes = fs::read(file_path).unwrap();
    let image = graphics::Image::from_bytes(ctx, &image_bytes).unwrap();
    image
}



