use std::ops::Add;

use macroquad::prelude::*;

const SCREEN_WIDTH: f32 = 600.0;
const SCREEN_HEIGHT: f32 = 400.0;
const ASTEROID_MAX_SIZE: i8 = 3;
const ASTEROID_VELOCITY: f32 = 10.0;
// const UNITS: f32 = 32.0;
const UNITS: f32 = 16.0;
const PLAYER_WIDTH: f32 = 1.0;
const PLAYER_HEIGHT: f32 = 1.0;
const BULLET_WIDTH: f32 = 4.0;
const BULLET_LIVE_TIME: f64 = 2.0;
const TURRET_COOLDOWN: f64 = 0.5;
const PLAYER_ACCELERATION: f32 = 10.0;

enum RunState {
    Menu,
    Running,
    GameOver,
}

#[derive(Clone)]
struct Asteroid {
    pos: Vec2,
    vel: Vec2,
    size: i8,
}

struct Bullet {
    pos: Vec2,
    created_at: f64,
    vel: Vec2,
}

struct Spaceship {
    w: f32,
    h: f32,
    pos: Vec2,
    angle: f32,
    vel: Vec2,
    acc: f32,
    last_turret_frame: f64,
}
struct GameState {
    scl: f32, // scale
    player: Spaceship,
    bullets: Vec<Bullet>,
    asteroids: Vec<Asteroid>,
    lives: i8,
    run_state: RunState,
}

fn handle_input(gs: &mut GameState) {
    let delta = get_frame_time();
    let rotation = gs.player.angle.to_radians();
    let sh = gs.player.h * gs.scl; // ship height

    match gs.run_state {
        RunState::Running => {
            if is_key_down(KeyCode::Left) {
                gs.player.angle = -((gs.player.angle - 5.0).abs() % 360.0);
            }
            if is_key_down(KeyCode::Right) {
                gs.player.angle = (gs.player.angle + 5.0) % 360.0;
            }

            if is_key_down(KeyCode::Up) {
                gs.player.vel = vec2(
                    gs.player.vel.x + (7.0 * delta) * rotation.sin(),
                    gs.player.vel.y - (7.0 * delta) * rotation.cos(),
                );
            }
            if is_key_down(KeyCode::Down) {
                gs.player.vel = vec2(
                    gs.player.vel.x - (7.0 * delta) * rotation.sin(),
                    gs.player.vel.y + (7.0 * delta) * rotation.cos(),
                );
            }
            let time = get_time();
            if is_key_down(KeyCode::Space) && time - gs.player.last_turret_frame > TURRET_COOLDOWN {
                gs.player.last_turret_frame = time;
                gs.bullets.push(Bullet {
                    pos: vec2(
                        gs.player.pos.x + rotation.sin() * (sh / 2.),
                        gs.player.pos.y - rotation.cos() * (sh / 2.),
                    ),
                    created_at: time,
                    vel: gs.player.vel + vec2(200.0 * rotation.sin(), -(200.0 * rotation.cos())),
                })
            }
        }
        _ => {}
    }
}

fn wrap(pos: Vec2, width: f32, height: f32) -> Vec2 {
    let mut new_pos = pos;
    if pos.x > screen_width() {
        new_pos.x = 0.0 - width;
    } else if new_pos.x < 0.0 - width {
        new_pos.x = screen_width();
    }

    if new_pos.y > screen_height() {
        new_pos.y = 0.0 - height;
    } else if new_pos.y < 0.0 - height {
        new_pos.y = screen_height();
    }

    new_pos
}

fn update(gs: &mut GameState) {
    let delta = get_frame_time();
    let time = get_time();
    match gs.run_state {
        RunState::Running => {
            let rotation = gs.player.angle.to_radians();
            gs.player.pos = gs.player.pos + gs.player.vel;
            // apply space friction
            let mut new_vel = gs.player.vel;
            if gs.player.vel.x > 0.0 {
                new_vel.x = clamp(gs.player.vel.x - (1.0 * delta), 0.0, gs.player.vel.x);
            } else {
                new_vel.x = clamp(gs.player.vel.x + (1.0 * delta), gs.player.vel.x, 0.0);
            };
            if gs.player.vel.y > 0.0 {
                new_vel.y = clamp(gs.player.vel.y - (1.0 * delta), 0.0, gs.player.vel.y);
            } else {
                new_vel.y = clamp(gs.player.vel.y + (1.0 * delta), gs.player.vel.y, 0.0);
            };
            gs.player.vel = new_vel;

            // update bullets and handle bounds
            for bullet in gs.bullets.iter_mut() {
                bullet.pos = wrap(
                    bullet.pos + (bullet.vel * delta),
                    BULLET_WIDTH,
                    BULLET_WIDTH,
                );
            }
            gs.bullets
                .retain(|b| time - b.created_at < BULLET_LIVE_TIME);

            // handle bounds
            gs.player.pos = wrap(gs.player.pos, gs.player.w, gs.player.h)
        }
        _ => {}
    }
}

fn draw_spaceship(ship: &Spaceship, scl: f32) {
    let Spaceship {
        pos,
        vel,
        angle,
        w,
        h,
        ..
    } = ship;

    let rotation = angle.to_radians();
    let sh = h * scl; // ship height
    let sw = w * scl; // ship width

    let v1 = Vec2::new(
        ship.pos.x + rotation.sin() * sh / 2.,
        ship.pos.y - rotation.cos() * sh / 2.,
    );
    let v2 = Vec2::new(
        ship.pos.x - rotation.cos() * sw / 2. - rotation.sin() * sh / 2.,
        ship.pos.y - rotation.sin() * sw / 2. + rotation.cos() * sh / 2.,
    );
    let v3 = Vec2::new(
        ship.pos.x + rotation.cos() * sw / 2. - rotation.sin() * sh / 2.,
        ship.pos.y + rotation.sin() * sw / 2. + rotation.cos() * sh / 2.,
    );

    draw_triangle_lines(v1, v2, v3, 1.0, BLACK);

    draw_circle(pos.x, pos.y, 0.1 * scl, RED);

    draw_line(pos.x, pos.y, pos.x + vel.x, pos.y + vel.y, 1.0, GREEN)
}

fn draw(gs: &GameState) {
    clear_background(WHITE);

    match gs.run_state {
        RunState::Running => {
            draw_spaceship(&gs.player, gs.scl);

            for bullet in gs.bullets.iter() {
                draw_circle(bullet.pos.x, bullet.pos.y, BULLET_WIDTH / 2.0, BLACK)
            }

            draw_text(
                &format!("{}", gs.player.vel.to_string()),
                100.0,
                100.0,
                30.0,
                BLACK,
            );
            draw_text(
                &format!("{}", gs.player.angle.to_string()),
                100.0,
                200.0,
                30.0,
                BLACK,
            );
        }
        _ => {}
    }
}

#[macroquad::main("Pong")]
async fn main() {
    let mut gs = GameState {
        run_state: RunState::Running,
        scl: screen_height() / UNITS,
        player: Spaceship {
            w: PLAYER_WIDTH,
            h: PLAYER_HEIGHT,
            pos: vec2(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0),
            angle: 0.0,
            vel: vec2(0.0, 0.0),
            acc: 0.0,
            last_turret_frame: 0.0,
        },
        asteroids: vec![
            Asteroid {
                pos: vec2(0.0, 0.0),
                vel: vec2(ASTEROID_VELOCITY, ASTEROID_VELOCITY),
                size: 3
            };
            6
        ],
        bullets: Vec::new(),
        lives: 5,
    };

    request_new_screen_size(SCREEN_WIDTH, SCREEN_HEIGHT);

    loop {
        gs.scl = screen_height() / UNITS;

        handle_input(&mut gs);
        update(&mut gs);
        draw(&gs);

        next_frame().await
    }
}
