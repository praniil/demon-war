use ggez::mint::{Point2, Vector2};
use ggez::{Context, GameResult};
use ggez::event::EventHandler;
use ggez::graphics::{self, drawable_size, Color, DrawMode, DrawParam, Mesh, Rect};
use glam::Vec2;
use core::time;
use std::thread::current;
use std::{clone, thread};
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

struct SpiderNet {
    size: (f32, f32),
    position: (f32, f32),
}

struct Spider {
    size: (f32, f32),
    position: (f32, f32),
    moving_left: bool,
    health_point: HpMeter,
}

impl Spider {
    fn update_position(&mut self) ->  f32 {
        if self.moving_left {
            self.position.0 -= 5.0;
            if self.position.0 <= 0.0 + self.size.0 * 3.0 {
                self.position.0 += 5.0;
                self.moving_left = false;
            } 
        }
        if !self.moving_left {
            self.position.0 += 5.0;
            if self.position.0 >= 1920.0 - self.size.0 * 3.0{
                self.position.0 -= 5.0;
                self.moving_left = true;
            }
        }
        
        self.position.0
    }
}

impl Gravity for SpiderNet{
    fn gravity(&mut self) {
        self.position.1 += 6.0;
        //840.0 position of hero in y
        if self.position.1 > DEFAULT_POS_HERO {
            self.position.1 = DEFAULT_POS_HERO;
        }
    }
}

struct Dinosaur {
    size : (f32, f32),
    default_position: (f32, f32),
    current_position: (f32, f32),
    health_point: HpMeter,
}

impl Dinosaur{
    fn update_position(&mut self) -> (f32, f32){
        self.current_position.0 -= 2.5;
        if self.current_position.0 < 0.0 {
            self.current_position = self.default_position;
        }
        (self.current_position.0, self.current_position.1)
    }
}

// impl std::ops::Drop for Hero{
//     fn drop(&mut self) {
//         println!("hero dropped");
//     }
// }

struct Bat {
    size: (f32, f32),
    position: (f32, f32),
    health_point: HpMeter,
}

#[derive(Clone)]
struct AbilitiesIcon {
    size: (f32, f32),
    start_instant: Instant,
    charged: bool,
}

impl AbilitiesIcon {
    fn start_timer(&mut self) {
        println!("charged: {}", self.charged);

        if !self.charged {
            println!("inside !self.charged");
            let ability_recharge_duration = time::Duration::from_secs(5);
            if self.start_instant.elapsed() >= ability_recharge_duration {
                self.charged = true;
                println!("Ability charged!");
            } else {
                println!("Still charging...");
            }
        } else {
            println!("Using ability, resetting cooldown...");
            self.start_instant = Instant::now();
        }
    }
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

        self.position.0 += xinc * 2.0;
        self.position.1 += yinc * 2.0;

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
        let fill_height = meter_height - 5.0;
        (fill_width, fill_height)
    }
}

fn get_health_meter_dimension(character: &dyn HealthMeter) -> (f32, f32) {
    character.health_meter()
}

impl HealthMeter for Dinosaur {
    fn health_meter(&self) -> (f32, f32) {
        let health_percent = self.health_point.currrent / self.health_point.max;
        let meter_width = 90.0;
        let meter_height = 20.0;
        let fill_width = health_percent * meter_width;
        let fill_height = meter_height - 5.0;
        (fill_width, fill_height)
    }
}

impl HealthMeter for Spider {
    fn health_meter(&self) -> (f32, f32) {
        let health_percent = self.health_point.currrent / self.health_point.max;
        let meter_width = 80.0;
        let meter_height = 18.0;
        let fill_width = health_percent * meter_width;
        let fill_height = meter_height - 5.0;
        (fill_width, fill_height)
    }
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

impl Gravity for Dinosaur {
    fn gravity(&mut self) {
        self.current_position.1 += 6.0;
        //840.0 position of hero in y
        if self.current_position.1 > DEFAULT_POS_HERO {
            self.current_position.1 = DEFAULT_POS_HERO;
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
    // hero_arrow_dist_rect: Rect,
    // hero_knife_dist_rect: Rect,
    // arrow_rect: Rect,
    // ability_icon: AbilitiesIcon,
    dino_rect: Rect,
    bat_character: Rect,
    hero_character_arrows: Image,
    hero_character_knife: Image,
    bat_img : Image,
    dinosaur_image: Image,
    shield_img: Image,
    teleport_img: Image,
    ultimate_img: Image,
    dinosaur: Dinosaur,
    hero: Hero,
    bat: Bat,
    arrows: Vec<Arrow>,
    draw_shield: bool,
    shield_start_time: Option<Instant>,
    teleport: bool,
    hero_switch: bool,
    hero_arrow_bat_collision: bool,
    draw_hero: bool,
    draw_bat: bool,
    draw_hp_meter_hero: bool,
    draw_hp_meter_bat:bool,
    draw_hp_meter_dinosaur: bool,
    bat_inside_range: bool,
    use_knife_bat: bool,
    arrow_overlap_bat: bool,
    ultimate_increase_health: bool,
    dinosaur_hero_overlaps: bool,
    dino_inside_range: bool,
    use_knife_dino: bool,
    draw_dino: bool,
    spider: Spider,
    spider_net: SpiderNet,
    spider_rect: Rect,
    spider_net_rect: Rect,
    spider_img: Image,
    spider_net_img: Image,
    draw_spider: bool,
    draw_spider_net: bool,
    hero_inside_net: bool,
    last_draw_time_spider: Instant,
    hero_arrow_spider_collision: bool,
    draw_hp_meter_spider: bool,
    shield_ability: AbilitiesIcon,
    teleport_ability: AbilitiesIcon,
    ultimate_ability: AbilitiesIcon,
}

fn get_random_position() -> (f32, f32){
    let mut rng = rand::thread_rng();
    let random_x: f32 = rng.gen_range(40.0..1800.0);
    let random_y: f32 = rng.gen_range(40.0..600.0);
    (random_x, random_y)
}

impl PlayState {
    pub fn new(ctx: &mut Context) -> GameResult<PlayState> {
        let hero_character_arrows = load_image(ctx, "/home/pranil/rustProjects/demon_war/resources/copy_hero.png");
        let hero_character_knife = load_image(ctx, "/home/pranil/rustProjects/demon_war/resources/hero_knife.png");
        let dinosaur_image = load_image(ctx, "/home/pranil/rustProjects/demon_war/resources/dinosaur.png");
        let spider_img = load_image(ctx, "/home/pranil/rustProjects/demon_war/resources/spider.png");
        let spider_net_img = load_image(ctx, "/home/pranil/rustProjects/demon_war/resources/spider net.png");
        let shield_img = load_image(ctx, "/home/pranil/rustProjects/demon_war/resources/shield_800.png");
        let teleport_img = load_image(ctx, "/home/pranil/rustProjects/demon_war/resources/teleport.png");
        let ultimate_img = load_image(ctx, "/home/pranil/rustProjects/demon_war/resources/heal_ulti.png");
        let last_draw_time_spider = Instant::now();

        // let ability_icon = AbilitiesIcon {
        //     size: (70.0, 70.0),
        //     charged: false,
        //     start_instant: Instant::now(),
        // };

        let shield_ability = AbilitiesIcon {
            size: (70.0, 70.0),
            charged: false,
            start_instant: Instant::now(),
        };

        let teleport_ability = AbilitiesIcon {
            size: (70.0, 70.0),
            charged: false,
            start_instant: Instant::now(),
        };

        let ultimate_ability = AbilitiesIcon {
            size: (70.0, 70.0),
            charged: false,
            start_instant: Instant::now(),
        };

        let dinosaur = Dinosaur {
            default_position: (1820.0, 880.0),
            size: (140.0, 120.0),
            current_position: (1820.0, 880.0),
            health_point: HpMeter {
                max: 50.0,
                currrent: 50.0
            }
        };

        let hero = Hero {
            size: (95.0, 120.0),
            position: (1920.0 / 2.0, 840.0),
            health_point: HpMeter { max: 150.0, currrent: 150.0 },
        };

        let bat_img = load_image(ctx, "/home/pranil/rustProjects/demon_war/resources/bat.png");
        let bat = Bat {
            size: (80.0, 80.0),
            position: get_random_position(),
            health_point: HpMeter { max:50.0, currrent: 50.0 },
        };

        let spider = Spider {
            size: (80.0, 80.0),
            position: get_random_position(),
            moving_left: true,
            health_point: HpMeter { max: 50.0, currrent: 50.0 },
        };
        
        let spider_net = SpiderNet {
            size: (200.0, 150.0),
            position: (spider.position.0 - 5.0 , spider.position.1 + spider.size.1 + 20.0 )
        };
        
        let spider_rect = graphics::Rect::new(spider.position.0, spider.position.1, spider.size.0, spider.size.1);
        let spider_net_rect = graphics::Rect::new(spider_net.position.0, spider_net.position.1, spider_net.size.0, spider_net.size.1);
        let arrow = Arrow {
            position : (hero.position.0 - 47.0 + hero.size.0 / 2.0, hero.position.1),
            ongoing : false
        };
        // let arrow_rect = graphics::Rect::new(arrow.position.0 - 2.0 + hero.size.0 / 2.0, arrow.position.1 - 44.0, 2.0, 60.0);
        let arrows = vec![arrow];
        let draw_shield = false;
        let shield_start_time = None;
        let teleport = false;
        let hero_switch = false;
        let hero_arrow_bat_collision = false;
        let draw_hero = true;
        let draw_bat = true;
        let draw_hp_meter_hero = false;
        let draw_hp_meter_bat = false;
        let draw_hp_meter_dinosaur = false;
        let bat_inside_range= false;
        // let hero_arrow_dist_rect = graphics::Rect::new(hero.position.0, hero.position.1, hero.size.0, hero.size.1);
        // let hero_knife_dist_rect = graphics::Rect::new(hero.position.0, hero.position.1 + 15.0, hero.size.0, hero.size.1);
        let use_knife = false;
        let bat_character = graphics::Rect::new(bat.position.0, bat.position.1, bat.size.0, bat.size.1);
        let arrow_overlap_bat = false;
        let ultimate_increase_health = false;
        let dinosaur_hero_overlaps= false;
        let dino_inside_range = false;
        let dino_rect = graphics::Rect::new(dinosaur.current_position.0, dinosaur.current_position.1, dinosaur.size.0, dinosaur.size.1);
        let use_knife_dino= false;
        let draw_dino = true;
        let draw_spider= true;
        let draw_spider_net = false;
        let hero_inside_net = false;
        let hero_arrow_spider_collision = false;
        let draw_hp_meter_spider = false;
        
        Ok(PlayState{
            hero_character_arrows,
            hero_character_knife,
            bat_character,
            bat_img,
            dinosaur_image,
            dinosaur,
            arrows,
            hero,
            draw_shield,
            teleport,
            shield_start_time,
            hero_switch,
            bat,
            hero_arrow_bat_collision,
            draw_hero,
            draw_bat,
            draw_hp_meter_hero,
            draw_hp_meter_bat,
            draw_hp_meter_dinosaur,
            bat_inside_range,
            // hero_arrow_dist_rect,
            // hero_knife_dist_rect,
            use_knife_bat: use_knife,
            // arrow_rect,
            arrow_overlap_bat,
            ultimate_increase_health,
            dinosaur_hero_overlaps,
            dino_inside_range,
            dino_rect,
            use_knife_dino,
            draw_dino,
            spider,
            spider_net,
            spider_rect,
            spider_net_rect,
            spider_img,
            spider_net_img,
            draw_spider,
            draw_spider_net,
            last_draw_time_spider,
            hero_inside_net,
            hero_arrow_spider_collision,
            draw_hp_meter_spider,
            shield_img,
            teleport_img,
            ultimate_img,
            // ability_icon,
            shield_ability,
            teleport_ability,
            ultimate_ability,
        })
    }
}

impl EventHandler <ggez::GameError> for PlayState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        let mut gravity_objects: Vec<&mut dyn Gravity> = vec![&mut self.hero];
        gravity_objects.push(&mut self.dinosaur);
        if self.draw_spider_net {
            gravity_objects.push(&mut self.spider_net);
        }
        apply_gravity(&mut gravity_objects);

        //mapping the position of spider_net_Rect and spider net
        self.spider_net_rect.x = self.spider_net.position.0;
        self.spider_net_rect.y = self.spider_net.position.1;

        self.spider_rect.x = self.spider.update_position();
        //dinosaur attacking hero
        (self.dino_rect.x, self.dino_rect.y) = self.dinosaur.update_position();

        if self.dinosaur_hero_overlaps && self.draw_dino && self.draw_hero{
            self.draw_hp_meter_hero = true;
            self.draw_hp_meter_dinosaur = true;
            let hp = self.hero.health_point.currrent;
            let mut decrease_hp = 18.75;
            if hp < decrease_hp {
                decrease_hp = hp;
            }
            if !self.draw_shield {
                self.hero.health_point.currrent -= decrease_hp;
            }
            self.dinosaur_hero_overlaps = false;
            if self.dinosaur.current_position.0 > self.hero.position.0 {
                self.dinosaur.current_position.0 += 40.0;
            } else {
                self.dinosaur.current_position.1 -= 80.0;
                self.dinosaur.current_position.0 += 100.0;
            }
            if self.hero.health_point.currrent == 0.0 {
                self.draw_hero = false;
                self.draw_bat = false;
                self.draw_hp_meter_hero = false;
                self.draw_hp_meter_bat = false;
                self.draw_dino = false;
                self.draw_hp_meter_dinosaur = false;
                self.draw_shield = false;
                self.draw_spider = false;
                self.draw_spider_net = false;
            }
        }

        // spider net throwing
        if self.draw_spider{
            if self.last_draw_time_spider.elapsed() > time::Duration::new(4, 0) {
                self.spider_net.position = (self.spider.position.0 - 15.0, self.spider.position.1 + self.spider.size.1 + 20.0);
                self.draw_spider_net = true;
                self.last_draw_time_spider= Instant::now();
            }
            self.hero_inside_net = false;
        }
        
        if self.ultimate_increase_health {
            let mut increase_hp = 15.0;
            let difference_hp = self.hero.health_point.max - self.hero.health_point.currrent;
            if difference_hp < increase_hp {
                increase_hp = difference_hp;
            }
            self.hero.health_point.currrent += increase_hp;
            if self.hero.health_point.currrent == self.hero.health_point.max {
                self.hero.health_point.currrent = self.hero.health_point.max - 15.0;
                self.ultimate_increase_health = false;
                self.ultimate_ability.charged = false;
                // self.ultimate_ability.start_instant = Instant::now();
            }
        }


        //for hero with arrows
        if self.draw_hero {
            (self.bat.position.0, self.bat.position.1) = self.bat.update_bat_position(self.hero.position.0, self.hero.position.1);
            self.bat_character = graphics::Rect::new(self.bat.position.0, self.bat.position.1, self.bat.size.0, self.bat.size.1);
        }


        for arrow in &mut self.arrows {
            if arrow.ongoing {
                arrow.position.1 -= 15.5;
                if self.arrow_overlap_bat && self.draw_bat && self.draw_hero{
                    self.draw_hp_meter_bat = true;
                    let current_bat_hp = self.bat.health_point.currrent;
                    let mut decrease_hp = 25.0;
                    if current_bat_hp < decrease_hp {
                        decrease_hp = current_bat_hp;
                    }
                    self.bat.health_point.currrent -= decrease_hp;
                    if self.bat.health_point.currrent == 0.0 {
                        self.bat.position = get_random_position();
                        self.bat.health_point.currrent = self.bat.health_point.max;
                        self.draw_hp_meter_bat = false;
                    }
                    self.arrow_overlap_bat = false;
                    arrow.ongoing = false; 
                }
                
                if self.hero_arrow_spider_collision {
                    self.draw_hp_meter_spider = true;
                    let mut decrease_hp = 25.0;
                    let current_hp = self.spider.health_point.currrent;
                    if current_hp < decrease_hp {
                        decrease_hp = current_hp;
                    }
                    println!("current spider health: {}", self.spider.health_point.currrent);
                    self.spider.health_point.currrent -= decrease_hp;
                    if self.spider.health_point.currrent == 0.0 {
                        (self.spider_rect.x, self.spider_rect.y) = get_random_position();
                        self.spider.health_point.currrent = self.spider.health_point.max;
                        self.draw_hp_meter_spider = false;
                    }
                    self.hero_arrow_spider_collision = false;
                    arrow.ongoing = false;
                }

                if arrow.position.1 < 0.0 {
                    arrow.ongoing = false; 
                }
            }
        }
        
        
        if let Some(start_time) = self.shield_start_time {
            if start_time.elapsed() > time::Duration::new(5, 0) {
                self.shield_ability.charged = false;
                self.draw_shield = false;
                self.shield_start_time = None;
                // self.shield_ability.start_instant = Instant::now();
            }
        }

        if self.use_knife_bat && self.draw_bat && self.draw_hero{
            let current_bat_hp = self.bat.health_point.currrent;
            let mut decrease_hp = 25.0;
            if current_bat_hp < decrease_hp {
                decrease_hp = current_bat_hp;
            }
            self.bat.health_point.currrent -= decrease_hp;
            if self.bat.health_point.currrent == 0.0 {
                self.draw_hp_meter_bat = false;
                (self.bat.position.0, self.bat.position.1) = get_random_position();
                self.bat.health_point.currrent = self.bat.health_point.max;
            }
            self.use_knife_bat = false;
        }

        if self.use_knife_dino && self.draw_hero && self.draw_dino {
            self.draw_hp_meter_dinosaur = true;
            let current_dino_hp = self.dinosaur.health_point.currrent;
            let mut decrease_hp = 25.0;
            if current_dino_hp < decrease_hp {
                decrease_hp = current_dino_hp;
            }
            self.dinosaur.health_point.currrent -= decrease_hp;
            if self.dinosaur.health_point.currrent == 0.0 {
                self.draw_hp_meter_dinosaur = false;
                self.dinosaur.current_position = self.dinosaur.default_position;
                self.dinosaur.health_point.currrent = self.dinosaur.health_point.max;
            }
            self.use_knife_dino = false;   
        }

        //bat attacking the hero
        if self.hero_arrow_bat_collision && self.draw_hero && self.draw_bat{
            self.draw_hp_meter_bat = true;
            self.draw_hp_meter_hero = true;
            let hp = self.hero.health_point.currrent;
            let mut decrease_hp = 10.0;
            if hp < decrease_hp {
                decrease_hp = hp;
            }
            if !self.draw_shield {
                self.hero.health_point.currrent -= decrease_hp;
            }
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
                self.draw_bat = false;
                self.draw_hp_meter_hero = false;
                self.draw_hp_meter_bat = false;
                self.draw_dino = false;
                self.draw_hp_meter_dinosaur = false;
                self.draw_shield = false;
                self.draw_spider = false;
                self.draw_spider_net = false;
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::from_rgb(240, 240, 240));
        //outside self.hero.swith == false so that the arrow goes on moving up when hero is switched with knife from arrows
        for arrow in &self.arrows {
            if arrow.ongoing == true {
                let arrow = graphics::Rect::new(arrow.position.0 - 2.0 + self.hero.size.0 / 2.0, arrow.position.1 - 44.0, 2.0, 60.0);
                let arrow_mess = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), arrow, Color::from_rgb(0, 102, 204)).unwrap();
                if arrow.overlaps(&self.bat_character) {
                    self.arrow_overlap_bat = true;
                }
                if arrow.overlaps(&self.spider_rect) {
                    self.hero_arrow_spider_collision = true;
                }
                graphics::draw(ctx, &arrow_mess, DrawParam::default()).unwrap();
            }
        }

        //shield icon
        let shield = graphics::Rect::new(520.0, 980.0, self.shield_ability.size.0, self.shield_ability.size.1);
        let shield_draw_param = DrawParam::default().dest([shield.x, shield.y]).scale([self.shield_ability.size.0 / self.shield_img.width() as f32, self.shield_ability.size.1 / self.shield_img.height() as f32]);
        graphics::draw(ctx, &self.shield_img, shield_draw_param).unwrap();
        self.shield_ability.start_timer();

        //teleport icon
        let teleport = graphics::Rect::new(920.0, 980.0, self.teleport_ability.size.0, self.teleport_ability.size.1);
        let teleport_draw_param = DrawParam::default().dest([teleport.x, teleport.y]).scale([self.teleport_ability.size.0 / self.teleport_img.width() as f32, self.teleport_ability.size.1 / self.teleport_img.height() as f32]);
        graphics::draw(ctx, &self.teleport_img, teleport_draw_param).unwrap();
        self.teleport_ability.start_timer();

        //ultimate icon
        let ultimate = graphics::Rect::new(1420.0, 980.0, self.ultimate_ability.size.0, self.ultimate_ability.size.1);
        let ultimate_draw_param = DrawParam::default().dest([ultimate.x, ultimate.y]).scale([self.ultimate_ability.size.0 / self.ultimate_img.width() as f32, self.ultimate_ability.size.1 / self.ultimate_img.height() as f32]);
        graphics::draw(ctx, &self.ultimate_img, ultimate_draw_param).unwrap();
        self.ultimate_ability.start_timer();


        // let bat_character = graphics::Rect::new(self.bat.position.0, self.bat.position.1, self.bat.size.0, self.bat.size.1);
        if self.draw_bat  {
            let bat_character = graphics::Rect::new(self.bat_character.x, self.bat_character.y, self.bat_character.w, self.bat_character.h);
            let bat_draw_param  = DrawParam::default().dest([bat_character.x, bat_character.y]).scale([self.bat.size.0 / self.bat_img.width() as f32, self.bat.size.1 / self.bat_img.width() as f32]);
            graphics::draw(ctx, &self.bat_img, bat_draw_param)?;
        }
        
        //draw dinosaur
        if self.draw_dino {
            let dinosaur_rect = self.dino_rect;
            // let dinosaur_rect = graphics::Rect::new(self.dinosaur.current_position.0, self.dinosaur.current_position.1, self.dinosaur.size.0, self.dinosaur.size.1);
            let dinosaur_draw_param = DrawParam::default().dest([dinosaur_rect.x, dinosaur_rect.y]).scale([self.dinosaur.size.0 / self.dinosaur_image.width() as f32, self.dinosaur.size.1 / self.dinosaur_image.height() as f32]);
            graphics::draw(ctx, &self.dinosaur_image, dinosaur_draw_param).unwrap();
        }
        
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
            let fill_rect = graphics::Rect::new(self.bat.position.0 + 1.0, self.bat.position.1 - 35.0, fill_width_bat, fill_height_bat);
            let fill_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), fill_rect, graphics::Color::from_rgb(144, 238, 144))?;
            graphics::draw(ctx, &fill_mesh, graphics::DrawParam::default())?;
        }

        if self.draw_hp_meter_dinosaur {
            let (fill_width_dino, fill_height_dino) = get_health_meter_dimension(&self.dinosaur);
            let meter_width_dino = 100.0;
            let meter_height_dino = 20.0;
            
            // Draw the background of the health meter
            let background_rect = graphics::Rect::new(self.dinosaur.current_position.0 + 5.0, self.dinosaur.current_position.1 + 134.0, meter_width_dino, meter_height_dino);
            let background_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), background_rect, graphics::Color::from_rgb(128, 128, 128))?;
            graphics::draw(ctx, &background_mesh, graphics::DrawParam::default())?;
            
            // Draw the filled part of the health meter
            let fill_rect = graphics::Rect::new(self.dinosaur.current_position.0 + 10.0, self.dinosaur.current_position.1 + 136.0, fill_width_dino, fill_height_dino);
            let fill_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), fill_rect, graphics::Color::from_rgb(144, 238, 144))?;
            graphics::draw(ctx, &fill_mesh, graphics::DrawParam::default())?;
        }

        //spider hp meter
        if self.draw_hp_meter_spider {
            let (fill_width_spider, fill_height_spider) = get_health_meter_dimension(&self.spider);
            let meter_width_spider = 100.0;
            let meter_height_spider = 20.0;
            
            // Draw the background of the health meter
            let background_rect = graphics::Rect::new(self.spider.position.0 + 5.0, self.spider.position.1 + 134.0, meter_width_spider, meter_height_spider);
            let background_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), background_rect, graphics::Color::from_rgb(128, 128, 128))?;
            graphics::draw(ctx, &background_mesh, graphics::DrawParam::default())?;
            
            // Draw the filled part of the health meter
            let fill_rect = graphics::Rect::new(self.spider.position.0 + 10.0, self.spider.position.1 + 136.0, fill_width_spider, fill_height_spider);
            let fill_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), fill_rect, graphics::Color::from_rgb(144, 238, 144))?;
            graphics::draw(ctx, &fill_mesh, graphics::DrawParam::default())?;
        }
        
        //shield for hero
        let radius = 80.0;
        
        if self.draw_shield == true {
            let circle_destn = convert_glam_to_point(glam::vec2(self.hero.position.0 + (self.hero.size.0 / 2.0), self.hero.position.1 + (self.hero.size.1 / 2.0)));
            let circle = Mesh::new_circle(ctx, graphics::DrawMode::fill(), circle_destn, radius, 1.0, Color::from_rgba(135, 206, 235, 120)).unwrap();
            graphics::draw(ctx, &circle, DrawParam::default()).unwrap();
        }

        //for hero with arrow
        if self.draw_hero { 
            let hero_arrow_dist_rect = graphics::Rect::new(self.hero.position.0, self.hero.position.1, self.hero.size.0, self.hero.size.1);
            let hero_knife_dist_rect = graphics::Rect::new(self.hero.position.0, self.hero.position.1 + 15.0, self.hero.size.0, self.hero.size.1);
            
            if self.hero_switch == false {
                let hero_draw_param  = DrawParam::default().dest([hero_arrow_dist_rect.x, hero_arrow_dist_rect.y]).scale([self.hero.size.0 / self.hero_character_arrows.width() as f32 , self.hero.size.1/ self.hero_character_arrows.height() as f32]);
                graphics::draw(ctx, &self.hero_character_arrows, hero_draw_param)?;
                
                if hero_arrow_dist_rect.overlaps(&self.bat_character) {
                    self.hero_arrow_bat_collision = true;
                }
            }
            //for hero with knife
            else {
                let hero_draw_param  = DrawParam::default().dest([hero_knife_dist_rect.x, hero_knife_dist_rect.y]).scale([self.hero.size.0 / self.hero_character_arrows.width() as f32 , self.hero.size.1/ self.hero_character_arrows.height() as f32]);
                graphics::draw(ctx, &self.hero_character_knife, hero_draw_param)?;
                
                //range where knife attack succeds
                let radius = 140.0;
                let circle_destn = convert_glam_to_point(glam::vec2(self.hero.position.0 + (self.hero.size.0 / 2.0), self.hero.position.1 - 35.0 + (self.hero.size.1 / 2.0)));
                let knife_range_mesh = Mesh::new_circle(ctx, graphics::DrawMode::stroke(5.0), circle_destn, radius, 2.0, Color::from_rgb(135, 206, 235)).unwrap();
                graphics::draw(ctx, &knife_range_mesh, DrawParam::default()).unwrap();
                
                let corners_bat = [
                    nalgebra::Point2::new(self.bat.position.0, self.bat.position.1),
                    nalgebra::Point2::new(self.bat.position.0, self.bat.position.1 + self.bat.size.1),
                    nalgebra::Point2::new(self.bat.position.0 + self.bat.size.0, self.bat.position.1),
                    nalgebra::Point2::new(self.bat.position.0 + self.bat.size.0, self.bat.position.1 + self.bat.size.1),
                ];
                    
                for corner in corners_bat {
                    let square_distance = (corner.x - circle_destn.x).powi(2) + (corner.y - circle_destn.y).powi(2);
                    if square_distance <= radius.powi(2) {
                        self.bat_inside_range = true
                    }
                }

                 let corners_dino = [
                    nalgebra::Point2::new(self.dinosaur.current_position.0, self.dinosaur.current_position.1),
                    nalgebra::Point2::new(self.dinosaur.current_position.0, self.dinosaur.current_position.1 + self.dinosaur.size.1),
                    nalgebra::Point2::new(self.dinosaur.current_position.0 + self.dinosaur.size.0, self.dinosaur.current_position.1),
                    nalgebra::Point2::new(self.dinosaur.current_position.0 + self.dinosaur.size.0, self.dinosaur.current_position.1 + self.dinosaur.size.1),
                ];
                    
                for corner in corners_dino {
                    let square_distance = (corner.x - circle_destn.x).powi(2) + (corner.y - circle_destn.y).powi(2);
                    if square_distance <= radius.powi(2) {
                        self.dino_inside_range = true
                    }
                }

            }
                
            if hero_arrow_dist_rect.overlaps(&self.bat_character) || hero_knife_dist_rect.overlaps(&self.bat_character) {
                self.hero_arrow_bat_collision = true;
            }
            if hero_arrow_dist_rect.overlaps(&self.bat_character) || hero_knife_dist_rect.overlaps(&self.bat_character) {
            self.hero_arrow_bat_collision = true;
            }
            if self.dino_rect.overlaps(&hero_arrow_dist_rect) || self.dino_rect.overlaps(&hero_knife_dist_rect) {
                self.dinosaur_hero_overlaps = true;
            }

            let hero_arrow_inside_net =  hero_arrow_dist_rect.x >= self.spider_net_rect.x && hero_arrow_dist_rect.y >= self.spider_net_rect.y && (hero_arrow_dist_rect.x + hero_arrow_dist_rect.w) <= (self.spider_net_rect.x + self.spider_net_rect.w) && 
            (hero_arrow_dist_rect.y + hero_arrow_dist_rect.h) <= (self.spider_net_rect.y + self.spider_net_rect.h);
            let hero_knife_insider_rect =  hero_knife_dist_rect.x >= self.spider_net_rect.x && hero_knife_dist_rect.y >= self.spider_net_rect.y && (hero_knife_dist_rect.x + hero_knife_dist_rect.w) <= (self.spider_net_rect.x + self.spider_net_rect.w) && 
            (hero_knife_dist_rect.y + hero_knife_dist_rect.h) <= (self.spider_net_rect.y + self.spider_net_rect.h);

            if hero_arrow_inside_net || hero_knife_insider_rect {
                self.hero_inside_net = true;
            }
        }

        //spider
        if self.draw_spider{
            let spider_rect = self.spider_rect;
            let spider_draw_param = DrawParam::default().dest([spider_rect.x, spider_rect.y]).scale([self.spider.size.0 / self.spider_img.width() as f32, self.spider.size.1 / self.spider_img.height() as f32]);
            graphics::draw(ctx, &self.spider_img, spider_draw_param).unwrap();
        }

        // spider net
        if self.draw_spider_net {
            let spider_net_rect = self.spider_net_rect;
            let spider_net_draw_param = DrawParam::default().dest([spider_net_rect.x, spider_net_rect.y]).scale([self.spider_net.size.0 / self.spider_net_img.width() as f32, self.spider_net.size.1 / self.spider_net_img.height() as f32]);
            graphics::draw(ctx, &self.spider_net_img, spider_net_draw_param).unwrap();
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
                } else {    //for hero with knife
                    if self.bat_inside_range{
                        let point_vec = glam::vec2(x, y);
                        let point = convert_glam_to_point(point_vec);
                        if self.bat_character.contains(point) {
                            self.use_knife_bat = true;
                        }
                    }
                    if self.dino_inside_range {
                        let point_vec = glam::vec2(x, y);
                        let point = convert_glam_to_point(point_vec);
                        if self.dino_rect.contains(point) {
                            self.use_knife_dino = true;
                        }
                    }
                }  
            }

            if self.teleport {
                if button == event::MouseButton::Right {
                    self.hero.position = (x, y);
                    self.teleport_ability.charged = false;
                    self.teleport = false;
                    // self.teleport_ability.start_instant = Instant::now();
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
            if !self.hero_inside_net {
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
                    _ => {}
                }
            } 
            match keycode {
                    KeyCode::Q => {
                        if self.shield_ability.charged {
                            self.draw_shield = true;
                            self.shield_start_time = Some(Instant::now());
                        }
                    }
                    KeyCode::E => {
                        if self.teleport_ability.charged {
                            self.teleport = true;
                        }
                    }
                    KeyCode::F => {
                        if self.ultimate_ability.charged {
                            self.ultimate_increase_health = true;
                        }
                    }
                    KeyCode::LControl => {
                        //false for hero with arrows and vice versa
                        if !self.hero_switch {
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

    


