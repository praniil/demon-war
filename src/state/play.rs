use ggez::mint::{Point2, Vector2};
use ggez::{Context, GameResult};
use ggez::event::EventHandler;
use ggez::graphics::{self, drawable_size, Color, DrawParam, Mesh, DrawMode};
use glam::Vec2;
use core::time;
use std::time::Instant;
use std::{fs, vec};
use ggez::graphics::Image;
use ggez::event;
use ggez::input::keyboard::{KeyCode, KeyMods};

const DEFAULT_POS_HERO: f32 = 840.0;

struct Arrow {
    position : (f32, f32),
    ongoing : bool,
}

struct Hero {
    size: (f32, f32),
    position: (f32, f32),
}

trait Gravity {
    fn gravity(&mut self);
}

impl Gravity for Hero {
    fn gravity(&mut self) {
        self.position.1 += 6.0;
        //840.0 position of hero in y
        if self.position.1 > DEFAULT_POS_HERO {
            self.position.1 = DEFAULT_POS_HERO;
        }
    }
}

fn apply_gravity(items: &mut Vec<&mut dyn Gravity>) {
    for item in items {
        item.gravity();
    }
}

fn convert_glam_to_point(vec: Vec2) -> Point2<f32> {
    Point2::from(vec.to_array())
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
    draw_shield: bool,
    shield_start_time: Option<Instant>,
    teleport: bool,
}


impl PlayState {
    pub fn new(ctx: &mut Context) -> GameResult<PlayState> {
        let hero_character = load_image(ctx, "/home/pranil/rustProjects/demon_war/resources/copy_hero.png");
        let hero = Hero {
            size: (95.0, 120.0),
            position: (1920.0 / 2.0, 840.0),
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
        let draw_shield = false;
        let shield_start_time = None;
        let teleport = false;

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
            draw_shield,
            teleport,
            shield_start_time,
        })
    }
}

impl EventHandler <ggez::GameError> for PlayState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        let mut gravity_objects: Vec<&mut dyn Gravity> = vec![&mut self.hero];
        apply_gravity(&mut gravity_objects);
        for arrow in &mut self.arrows {
            if arrow.ongoing == true {
                arrow.position.1 -= 10.5;
                if arrow.position.1 < 0.0 {
                    arrow.ongoing = false; 
                }
            }
        }
        if let Some(start_time) = self.shield_start_time {
            if start_time.elapsed() > time::Duration::new(5, 0) {
                self.draw_shield = false;
                self.shield_start_time = None;
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
                let arrow_mess = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), arrow, Color::from_rgb(0, 102, 204)).unwrap();
                graphics::draw(ctx, &arrow_mess, DrawParam::default()).unwrap();
            }
        }

        let radius = 80.0;

        if self.draw_shield == true {
            let circle_destn = convert_glam_to_point(glam::vec2(self.hero.position.0 + (self.hero.size.0 / 2.0), self.hero.position.1 + (self.hero.size.1 / 2.0)));
            let circle = Mesh::new_circle(ctx, graphics::DrawMode::fill(), circle_destn, radius, 1.0, Color::from_rgba(135, 206, 235, 120)).unwrap();
            graphics::draw(ctx, &circle, DrawParam::default()).unwrap();
        }
        graphics::present(ctx)?;

        Ok(())
    }

    fn mouse_button_down_event(
            &mut self,
            _ctx: &mut Context,
            button: event::MouseButton,
            x: f32,
            y: f32,
        ) {
        if button == event::MouseButton::Left {
            println!("left button in mouse clicked");
            let new_arrow = Arrow {
                position: (self.hero.position.0 - 47.0 + self.hero.size.0 / 2.0, self.hero.position.1),
                ongoing: true,
            };
            self.arrows.push(new_arrow);
        }

        if self.teleport {
            if button == event::MouseButton::Right {
                println!("x position of click: {} and y: {}", x, y);
                self.hero.position = (x, y);
                self.teleport = false;
            }
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
                self.hero.position.0 -= 80.0;
                if self.hero.position.0 < 0.0 {
                    self.hero.position.0 = 0.0;
                }
            }
            KeyCode :: W => {
                self.hero.position.1 -= 200.0;
                if self.hero.position.1 < 0.0 {
                    self.hero.position.1 = 0.0;
                }
                println!("pressed w");
            }
            KeyCode::D => {
                self.hero.position.0 += 80.0;
                if self.hero.position.0 > 1920.0 - self.hero.size.0 {
                    self.hero.position.0 = 1920.0 - self.hero.size.0;
                }
            }
            KeyCode::Q => {
                println!("pressed q");
                self.draw_shield = true;
                self.shield_start_time = Some(Instant::now());
            }
            KeyCode::E => {
                println!("pressed E");
                self.teleport = true;
            }
            KeyCode::F => {
                println!("ultimate");
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



