use super::spawner;
use macroquad::prelude::{screen_height, screen_width, vec2, Vec2};

pub const FONT_SIZE: f32 = 20.0;
pub const SCREEN_WIDTH: f32 = 800.0;
pub const SCREEN_HEIGHT: f32 = 600.0;
pub const ASTEROID_MAX_SIZE: f32 = 3.0;
pub const ASTEROID_VEL: f32 = 6.0;
pub const FRICT: f32 = 0.75;
//pub const UNITS: f32 = 32.0;
pub const UNITS: f32 = 16.0;
pub const MAX_PLAYER_LIVES: i32 = 3;
pub const PLAYER_WIDTH: f32 = 1.0;
pub const PLAYER_HEIGHT: f32 = 1.0;
pub const PLAYER_ACCL: f32 = 5.0;
pub const ANGLE_STEP: f32 = 5.0;
pub const BULLET_WIDTH: f32 = 0.1;
pub const BULLET_VEL: f32 = 300.0;
pub const BULLET_LIVE_TIME: f64 = 1.5; // in seconds
pub const TURRET_COOLDOWN: f64 = 0.5; // in seconds
pub const GAME_TIME: f32 = 100.0; // in seconds

#[derive(PartialEq)]
pub enum RunState {
    Menu,
    Running,
    Death,
    GameOver,
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
    pub fn get_points(&self) -> Vec<Vec2> {
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
    pub last_turret_frame: f64,
}

impl Spaceship {
    pub fn reset(&mut self) {
        self.vel = vec2(0.0, 0.0);
        self.angle = 0.0;
        self.pos = vec2(screen_width() / 2.0, screen_height() / 2.0);
    }

    pub fn get_points(&self, scale: f32) -> Vec<Vec2> {
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
    pub bullets: Vec<Bullet>,
    pub asteroids: Vec<Asteroid>,
    pub lives: i32,
    pub run_state: RunState,
    pub play_time: f32,
    pub score: i32,
    pub debug: bool,
}

pub fn get_new_game_state() -> GameState {
    let scale = screen_height() / UNITS;
    let center_pos = vec2(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0);

    let gs = GameState {
        run_state: RunState::Running,
        scl: scale,
        player: Spaceship {
            w: PLAYER_WIDTH,
            h: PLAYER_HEIGHT,
            pos: center_pos,
            angle: 0.0,
            vel: vec2(0.0, 0.0),
            last_turret_frame: 0.0,
        },
        asteroids: spawner::asteroids(
            center_pos,
            screen_width() / ASTEROID_MAX_SIZE,
            3,
            ASTEROID_MAX_SIZE,
            scale,
        ),
        bullets: Vec::new(),
        lives: MAX_PLAYER_LIVES,
        play_time: 0.0,
        score: 0,
        debug: false,
    };

    gs
}
