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
use rand::Rng;

const DEFAULT_POS_HERO: f32 = 840.0;
struct HpMeter {
    max: f32,
    currrent: f32,
}

struct Arrow {
    position : (f32, f32),
    ongoing : bool,
}

struct Hero {
    size: (f32, f32),
    position: (f32, f32),
    // health_point : u32,
    health_point: HpMeter,
}

// impl std::ops::Drop for Hero{
//     fn drop(&mut self) {
//         println!("hero dropped");
//     }
// }

struct Bat {
    size: (f32, f32),
    position: (f32, f32),
    // health_point : u32,
    health_point: HpMeter,
}

impl Bat {
    //dda algorithm to track the hero 
    fn update_bat_position(&mut self, hero_pos_x: f32, hero_pos_y: f32) -> (f32, f32) {
        let dx = hero_pos_x - self.position.0;
        let dy = hero_pos_y - self.position.1;

        let larger = dx.abs() > dy.abs();
        let steps;
        if larger {
            steps = dx.abs();
        } else {
            steps = dy.abs();
        }

        let xinc = dx / steps;
        let yinc = dy / steps;

        self.position.0 += xinc * 3.0;
        self.position.1 += yinc * 3.0;

        (self.position.0, self.position.1)
    }
}


trait HealthMeter {
    fn health_meter(&self) -> (f32, f32);
}

impl HealthMeter for Hero {
    fn health_meter(&self) -> (f32, f32) {
        let health_percent = self.health_point.currrent / self.health_point.max;
        let meter_width = 90.0;
        let meter_height = 20.0;
        let fill_width = health_percent * meter_width;
        let fill_height = meter_height - 4.0;
        (fill_width, fill_height)
    }
}

impl HealthMeter for Bat {
    fn health_meter(&self) -> (f32, f32) {
        let health_percent = self.health_point.currrent / self.health_point.max;
        let meter_width = 80.0;
        let meter_height = 18.0;
        let fill_width = health_percent * meter_width;
        let fill_height = meter_height - 3.0;
        (fill_width, fill_height)
    }
}

fn get_health_meter_dimension(character: &dyn HealthMeter) -> (f32, f32) {
    character.health_meter()
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
    hero_character_arrows: Image,
    hero_character_knife: Image,
    bat_img : Image,
    shield_ability_position: (f32, f32),
    shiled_ability_cooldown_position: (f32, f32),
    teleport_ability_position: (f32, f32),
    teleport_ability_cooldown_position: (f32, f32),
    ultimate_ability_position: (f32, f32),
    ultimate_ability_cooldown_position: (f32, f32),
    draw_arrow: bool,
    hero: Hero,
    bat: Bat,
    arrows: Vec<Arrow>,
    draw_shield: bool,
    shield_start_time: Option<Instant>,
    teleport: bool,
    hero_switch: bool,
    hero_arrow_bat_collision: bool,
    hero_knife_bat_collision: bool,
    draw_hero: bool,
    draw_hp_meter_hero: bool,
    draw_hp_meter_bat:bool,
}

fn get_random_position() -> (f32, f32){
    let mut rng = rand::thread_rng();
    let random_x: f32 = rng.gen_range(40.0..1800.0);
    let random_y: f32 = rng.gen_range(40.0..800.0);
    (random_x, random_y)
}

impl PlayState {
    pub fn new(ctx: &mut Context) -> GameResult<PlayState> {
        let hero_character_arrows = load_image(ctx, "/home/pranil/rustProjects/demon_war/resources/copy_hero.png");
        let hero_character_knife = load_image(ctx, "/home/pranil/rustProjects/demon_war/resources/hero_knife.png");
        let hero = Hero {
            size: (95.0, 120.0),
            position: (1920.0 / 2.0, 840.0),
            health_point: HpMeter { max:150.0, currrent: 150.0 },
            // health_point: 150,
        };

        let bat_img = load_image(ctx, "/home/pranil/rustProjects/demon_war/resources/bat.png");
        let bat = Bat {
            size: (80.0, 80.0),
            position: get_random_position(),
            health_point: HpMeter { max:50.0, currrent: 50.0 },
            // health_point: 50,
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
        let hero_switch = false;
        let hero_arrow_bat_collision = false;
        let hero_knife_bat_collision = false;
        let draw_hero = true;
        let draw_hp_meter_hero = false;
        let draw_hp_meter_bat = false;

        Ok(PlayState{
            hero_character_arrows,
            hero_character_knife,
            bat_img,
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
            hero_switch,
            bat,
            hero_knife_bat_collision,
            hero_arrow_bat_collision,
            draw_hero,
            draw_hp_meter_hero,
            draw_hp_meter_bat,
        })
    }
}

impl EventHandler <ggez::GameError> for PlayState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        let mut gravity_objects: Vec<&mut dyn Gravity> = vec![&mut self.hero];
        apply_gravity(&mut gravity_objects);
        //for hero with arrows
        
        if self.draw_hero {
            (self.bat.position.0, self.bat.position.1) = self.bat.update_bat_position(self.hero.position.0, self.hero.position.1);
        }

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

        if self.hero_arrow_bat_collision && self.draw_hero {
            self.draw_hp_meter_hero = true;
            self.draw_hp_meter_bat = true;
            let hp = self.hero.health_point.currrent;
            let mut decrease_hp = 15.0;
            if hp < decrease_hp {
                decrease_hp = hp;
            }
            self.hero.health_point.currrent -= decrease_hp;
            self.hero_arrow_bat_collision = false;
            if self.bat.position.0 > self.hero.position.0 {
                self.bat.position.0 -= 40.0;
                self.bat.position.1 -= 40.0;
            } else {
                self.bat.position.0 += 40.0;
                self.bat.position.1 -= 40.0;
            }

            if self.hero.health_point.currrent == 0.0 {
                self.draw_hero = false;
                self.draw_hp_meter_hero = false;
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::from_rgb(211, 211, 211));
        //outside self.hero.swith == false so that the arrow goes on moving up when hero is switched with knife from arrows
        for arrow in &self.arrows {
            if arrow.ongoing == true {
                let arrow = graphics::Rect::new(arrow.position.0 - 2.0 + self.hero.size.0 / 2.0, arrow.position.1 - 44.0, 2.0, 60.0);
                let arrow_mess = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), arrow, Color::from_rgb(0, 102, 204)).unwrap();
                graphics::draw(ctx, &arrow_mess, DrawParam::default()).unwrap();
            }
        }

        let bat_character = graphics::Rect::new(self.bat.position.0, self.bat.position.1, self.bat.size.0, self.bat.size.1);
        let bat_draw_param  = DrawParam::default().dest([bat_character.x, bat_character.y]).scale([self.bat.size.0 / self.bat_img.width() as f32, self.bat.size.1 / self.bat_img.width() as f32]);
        graphics::draw(ctx, &self.bat_img, bat_draw_param)?;

        /*hero hp meter*/
    
        if self.draw_hp_meter_hero {
            let (fill_width_hero, fill_height_hero) = get_health_meter_dimension(&self.hero);

            let meter_width_hero = 90.0;
            let meter_height_hero = 20.0;

        // Draw the background of the health meter
            let background_rect = graphics::Rect::new(self.hero.position.0 + 5.0, self.hero.position.1 + 134.0, meter_width_hero, meter_height_hero);
            let background_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), background_rect, graphics::Color::from_rgb(128, 128, 128))?;
            graphics::draw(ctx, &background_mesh, graphics::DrawParam::default())?;

        // Draw the filled part of the health meter
            let fill_rect = graphics::Rect::new(self.hero.position.0 + 10.0, self.hero.position.1 + 136.0, fill_width_hero, fill_height_hero);
            let fill_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), fill_rect, graphics::Color::from_rgb(144, 238, 144))?;
            graphics::draw(ctx, &fill_mesh, graphics::DrawParam::default())?;
        }

        /* bat hp meter */
        if self.draw_hp_meter_bat {
            let (fill_width_bat, fill_height_bat) = get_health_meter_dimension(&self.bat);
            let meter_width_bat = 80.0;
            let meter_height_bat = 18.0;

            //draw background
            let background_rect = graphics::Rect::new(self.bat.position.0, self.bat.position.1 - 34.0, meter_width_bat, meter_height_bat);
            let background_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), background_rect, graphics::Color::from_rgb(128, 128, 128))?;
            graphics::draw(ctx, &background_mesh, graphics::DrawParam::default())?;

            //draw filled part
            let fill_rect = graphics::Rect::new(self.bat.position.0 + 2.0, self.bat.position.1 - 36.0, fill_width_bat, fill_height_bat);
            let fill_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), fill_rect, graphics::Color::from_rgb(144, 238, 144))?;
            graphics::draw(ctx, &fill_mesh, graphics::DrawParam::default())?;
        }

        //for hero with arrow
        if self.draw_hero { 
            let hero_arrow_dist_rect = graphics::Rect::new(self.hero.position.0, self.hero.position.1, self.hero.size.0, self.hero.size.1);
            let hero_knife_dist_rect = graphics::Rect::new(self.hero.position.0, self.hero.position.1 + 15.0, self.hero.size.0, self.hero.size.1);

            if self.hero_switch == false {
                let hero_draw_param  = DrawParam::default().dest([hero_arrow_dist_rect.x, hero_arrow_dist_rect.y]).scale([self.hero.size.0 / self.hero_character_arrows.width() as f32 , self.hero.size.1/ self.hero_character_arrows.height() as f32]);
                graphics::draw(ctx, &self.hero_character_arrows, hero_draw_param)?;


                let radius = 80.0;

                if self.draw_shield == true {
                    let circle_destn = convert_glam_to_point(glam::vec2(self.hero.position.0 + (self.hero.size.0 / 2.0), self.hero.position.1 + (self.hero.size.1 / 2.0)));
                    let circle = Mesh::new_circle(ctx, graphics::DrawMode::fill(), circle_destn, radius, 1.0, Color::from_rgba(135, 206, 235, 120)).unwrap();
                    graphics::draw(ctx, &circle, DrawParam::default()).unwrap();
                }

                if hero_arrow_dist_rect.overlaps(&bat_character) {
                    self.hero_arrow_bat_collision = true;
                }
            }
            //for hero with knife
            else {
                let hero_draw_param  = DrawParam::default().dest([hero_knife_dist_rect.x, hero_knife_dist_rect.y]).scale([self.hero.size.0 / self.hero_character_arrows.width() as f32 , self.hero.size.1/ self.hero_character_arrows.height() as f32]);
                graphics::draw(ctx, &self.hero_character_knife, hero_draw_param)?;

                //range where knife attack succeds
                let radius = 120.0;
                let circle_destn = convert_glam_to_point(glam::vec2(self.hero.position.0 + (self.hero.size.0 / 2.0), self.hero.position.1 - 15.0 + (self.hero.size.1 / 2.0)));
                let knife_range_mesh = Mesh::new_circle(ctx, graphics::DrawMode::stroke(5.0), circle_destn, radius, 2.0, Color::from_rgb(135, 206, 235)).unwrap();
                graphics::draw(ctx, &knife_range_mesh, DrawParam::default()).unwrap();
            }

            if hero_arrow_dist_rect.overlaps(&bat_character) || hero_knife_dist_rect.overlaps(&bat_character) {
                self.hero_arrow_bat_collision = true;
            }

            if hero_arrow_dist_rect.overlaps(&bat_character) || hero_knife_dist_rect.overlaps(&bat_character) {
                self.hero_arrow_bat_collision = true;
            }
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
            //for hero with arrows
            if self.hero_switch == false {
                let new_arrow = Arrow {
                    position: (self.hero.position.0 - 47.0 + self.hero.size.0 / 2.0, self.hero.position.1),
                    ongoing: true,
                };
                self.arrows.push(new_arrow);
            }
        }

        if self.teleport {
            if button == event::MouseButton::Right {
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
            KeyCode::S => {
                self.hero.position.1 += 80.0;
                if self.hero.position.1 > DEFAULT_POS_HERO {
                    self.hero.position.1 = DEFAULT_POS_HERO;
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
            KeyCode::LControl => {
                //false for hero with arrows and vice versa
                if self.hero_switch == false {
                    self.hero_switch = true;
                } else {
                    self.hero_switch = false;
                }
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



