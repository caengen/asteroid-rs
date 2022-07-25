use super::spawner;
use macroquad::{
    audio::Sound,
    color_u8,
    prelude::{const_vec2, get_time, rand, screen_height, screen_width, vec2, Color, Vec2},
};

// vertical scale units. Screen height is 1:16
pub const UNITS: f32 = 16.0;

//colors
pub const BG_COLOR: Color = color_u8!(49, 47, 40, 1);

//dimensions
pub const POINT_FONT_SIZE: f32 = 40.0;
pub const FONT_SIZE: f32 = 20.0;
pub const SCREEN_WIDTH: f32 = 400.0;
pub const SCREEN_HEIGHT: f32 = 300.0;
pub const ASTEROID_MAX_SIZE: f32 = 3.0;
pub const PLAYER_WIDTH: f32 = 1.0;
pub const PLAYER_HEIGHT: f32 = 1.0;
pub const BULLET_WIDTH: f32 = 0.1;

//velocity
pub const GRAVITY: Vec2 = const_vec2!([0.0, 9.81]);
pub const PLAYER_ACCL: f32 = 7.5;
pub const PLAYER_MAX_VEL: f32 = 25.0;
pub const BULLET_VEL: f32 = 600.0;
pub const EXHAUST_VEL: f32 = 150.0;
pub const ASTEROID_VEL: f32 = 6.0;
pub const FRICT: f32 = 0.75;
pub const ANGLE_STEP: f32 = 4.0;

//time in seconds
pub const BULLET_LIVE_TIME: f64 = 0.75;
pub const TURRET_COOLDOWN: f64 = 0.5;
pub const EXHAUST_COOLDOWN: f64 = 0.175;
pub const EXHAUST_LIVE_TIME: f64 = 2.0;
pub const EXPLOSION_LIVE_TIME: f64 = 0.333;
pub const FLYING_POINT_LIVE_TIME: f64 = 0.666;
pub const GAME_TIME: f32 = 100.0;
pub const COMBO_TIMER: f32 = 3.0;

pub const MAX_PLAYER_LIVES: i32 = 3;
pub const SCORE_BASE: i32 = 16;

#[derive(PartialEq)]
pub enum RunState {
    Menu,
    Running,
    Death,
    StageComplete,
}

pub struct FlyingPoint {
    pub pos: Vec2,
    pub vel: Vec2,
    pub val: i32,
    pub created_at: f64,
}
pub struct Explosion {
    pub pos: Vec2,
    pub width: f32,
    pub size: f32,
    pub created_at: f64,
}

pub struct Star {
    pub pos: Vec2,
    pub size: f32,
}

impl Explosion {
    pub fn new(x: f32, y: f32, width: f32, size: f32) -> Self {
        Explosion {
            pos: vec2(x, y),
            width,
            size,
            created_at: get_time(),
        }
    }
}

#[derive(Clone)]
pub struct Asteroid {
    pub pos: Vec2,
    pub vel: Vec2,
    pub angle: f32,
    pub size: f32,
    pub points: Vec<Vec2>,
    pub w: f32,
    pub collision: bool,
}

impl Asteroid {
    pub fn points(&self) -> Vec<Vec2> {
        let rot = self.angle.to_radians();
        let c = rot.cos();
        let s = rot.sin();
        let mut points = Vec::new();
        self.points.iter().for_each(|p| {
            points.push(vec2(
                self.pos.x + p.x * c - p.y * s,
                self.pos.y + p.x * s + p.y * c,
            ));
        });

        points
    }
}

pub struct Exhaust {
    pub pos: Vec2,
    pub created_at: f64,
    pub vel: Vec2,
    pub size: f32,
}

pub struct Bullet {
    pub pos: Vec2,
    pub created_at: f64,
    pub vel: Vec2,
    pub collision: bool,
}

pub struct Spaceship {
    pub w: f32,
    pub h: f32,
    pub pos: Vec2,
    pub angle: f32,
    pub vel: Vec2,
    pub strafing: (bool, bool),
    pub last_turret_frame: f64,
    pub last_exhaust_frame: f64,
}

impl Spaceship {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Spaceship {
            w,
            h,
            pos: vec2(x, y),
            angle: 0.0,
            vel: vec2(0.0, 0.0),
            strafing: (false, false),
            last_turret_frame: 0.0,
            last_exhaust_frame: 0.0,
        }
    }
    pub fn reset(&mut self) {
        self.vel = vec2(0.0, 0.0);
        self.angle = 0.0;
        self.pos = vec2(screen_width() / 2.0, screen_height() / 2.0);
    }

    pub fn points(&self, scale: f32) -> Vec<Vec2> {
        let rot = self.angle.to_radians();
        let sh = self.h * scale; // ship height
        let sw = self.w * scale; // ship width

        let v1 = vec2(
            self.pos.x + rot.sin() * sh / 2.,
            self.pos.y - rot.cos() * sh / 2.,
        );
        let v2 = vec2(
            self.pos.x - rot.cos() * sw / 2. - rot.sin() * sh / 2.,
            self.pos.y - rot.sin() * sw / 2. + rot.cos() * sh / 2.,
        );
        let v3 = vec2(
            self.pos.x + rot.cos() * sw / 2. - rot.sin() * sh / 2.,
            self.pos.y + rot.sin() * sw / 2. + rot.cos() * sh / 2.,
        );
        let v4 = vec2(
            self.pos.x - rot.cos() * sw / 1.5 - rot.sin() * sh / 1.5,
            self.pos.y - rot.sin() * sw / 1.5 + rot.cos() * sh / 1.5,
        );
        let v5 = vec2(
            self.pos.x + rot.cos() * sw / 1.5 - rot.sin() * sh / 1.5,
            self.pos.y + rot.sin() * sw / 1.5 + rot.cos() * sh / 1.5,
        );

        vec![v1, v2, v3, v4, v5]
    }
}

pub struct GameState {
    pub scl: f32, // scale
    pub player: Spaceship,
    pub flying_points: Vec<FlyingPoint>,
    pub background: Vec<Star>,
    pub exhaust: Vec<Exhaust>,
    pub explosions: Vec<Explosion>,
    pub bullets: Vec<Bullet>,
    pub asteroids: Vec<Asteroid>,
    pub lives: i32,
    pub run_state: RunState,
    pub play_time: f32,
    pub combo_time: f32,
    pub combo: i32,
    pub score_multiplier: i32,
    pub score: i32,
    pub debug: bool,
    pub sounds: Vec<Option<Sound>>,
}

pub fn get_new_game_state() -> GameState {
    let scale = screen_height() / UNITS;
    let center_pos = vec2(screen_width() / 2.0, screen_height() / 2.0);

    let gs = GameState {
        asteroids: spawner::asteroids(
            center_pos,
            screen_width() / ASTEROID_MAX_SIZE,
            3,
            ASTEROID_MAX_SIZE,
            scale,
        ),
        background: spawner::stars(50, screen_width(), screen_height()),
        bullets: Vec::new(),
        combo: 0,
        combo_time: 0.0,
        debug: false,
        exhaust: Vec::new(),
        explosions: Vec::new(),
        flying_points: Vec::new(),
        lives: MAX_PLAYER_LIVES,
        play_time: 0.0,
        player: Spaceship::new(center_pos.x, center_pos.y, PLAYER_WIDTH, PLAYER_HEIGHT),
        run_state: RunState::Running,
        scl: scale,
        score: 0,
        score_multiplier: 1,
        sounds: vec![None; 10],
    };

    gs
}
