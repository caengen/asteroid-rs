use std::ops::Add;

use macroquad::prelude::*;

const SCREEN_WIDTH: f32 = 600.0;
const SCREEN_HEIGHT: f32 = 400.0;
const ASTEROID_MAX_SIZE: i8 = 3;
const ASTEROID_VELOCITY: f32 = 10.0;
const FRICT: f32 = 0.75;
// const UNITS: f32 = 32.0;
const UNITS: f32 = 16.0;
const PLAYER_WIDTH: f32 = 1.0;
const PLAYER_HEIGHT: f32 = 1.0;
const PLAYER_ACCL: f32 = 7.0;
const ANGLE_STEP: f32 = 5.0;
const BULLET_WIDTH: f32 = 4.0;
const BULLET_VEL: f32 = 300.0;
const BULLET_LIVE_TIME: f64 = 2.0;
const TURRET_COOLDOWN: f64 = 0.5;

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
    points: Vec<Vec2>,
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
                gs.player.angle = -((gs.player.angle - ANGLE_STEP).abs() % 360.0);
            }
            if is_key_down(KeyCode::Right) {
                gs.player.angle = (gs.player.angle + ANGLE_STEP) % 360.0;
            }

            if is_key_down(KeyCode::Up) {
                gs.player.vel = vec2(
                    gs.player.vel.x + (PLAYER_ACCL * delta) * rotation.sin(),
                    gs.player.vel.y - (PLAYER_ACCL * delta) * rotation.cos(),
                );
            }
            if is_key_down(KeyCode::Down) {
                gs.player.vel = vec2(
                    gs.player.vel.x - PLAYER_ACCL * delta * rotation.sin(),
                    gs.player.vel.y + PLAYER_ACCL * delta * rotation.cos(),
                );
            }
            let time = get_time();
            if is_key_down(KeyCode::Space) && time - gs.player.last_turret_frame > TURRET_COOLDOWN {
                gs.player.last_turret_frame = time;
                gs.bullets.push(Bullet {
                    pos: vec2(
                        gs.player.pos.x + rotation.sin() * sh / 2.,
                        gs.player.pos.y - rotation.cos() * sh / 2.,
                    ),
                    created_at: time,
                    vel: vec2(BULLET_VEL * rotation.sin(), -(BULLET_VEL * rotation.cos())),
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

fn get_points(origo: Vec2, amount: i32, size: f32) -> Vec<Vec2> {
    let mut points = Vec::new();
    let angle_inc = 360.0 / amount as f32;
    for i in 1..=amount {
        let rot = (angle_inc * i as f32).to_radians();
        let r = rand::gen_range(0.5, 1.0);
        points.push(vec2(
            origo.x + PLAYER_WIDTH * r * size * rot.sin(),
            origo.y - PLAYER_WIDTH * r * size * rot.cos(),
        ));
    }

    points
}

fn update(gs: &mut GameState) {
    let delta = get_frame_time();
    let time = get_time();
    match gs.run_state {
        RunState::Running => {
            gs.player.pos = gs.player.pos + gs.player.vel;
            // apply space friction
            let mut new_vel = gs.player.vel;
            if gs.player.vel.x > 0.0 {
                new_vel.x = clamp(gs.player.vel.x - (FRICT * delta), 0.0, gs.player.vel.x);
            } else {
                new_vel.x = clamp(gs.player.vel.x + (FRICT * delta), gs.player.vel.x, 0.0);
            };
            if gs.player.vel.y > 0.0 {
                new_vel.y = clamp(gs.player.vel.y - (FRICT * delta), 0.0, gs.player.vel.y);
            } else {
                new_vel.y = clamp(gs.player.vel.y + (FRICT * delta), gs.player.vel.y, 0.0);
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

            // handle player bounds
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

    // draw_circle(pos.x, pos.y, 0.1 * scl, RED);

    // vel vector debugging
    draw_line(
        pos.x,
        pos.y,
        pos.x + vel.x * scl / 2.0,
        pos.y + vel.y * scl / 2.0,
        1.0,
        GREEN,
    );
}

fn draw(gs: &GameState) {
    clear_background(WHITE);

    match gs.run_state {
        RunState::Running => {
            // draw_spaceship(&gs.player, gs.scl);

            for bullet in gs.bullets.iter() {
                draw_circle(bullet.pos.x, bullet.pos.y, BULLET_WIDTH / 2.0, BLACK)
            }

            for asteroid in gs.asteroids.iter() {
                for i in 0..=(asteroid.points.len() - 1) {
                    let p1 = asteroid.points[i];
                    let p2 = asteroid.points[(i + 1) % asteroid.points.len()];
                    draw_line(p1.x, p1.y, p2.x, p2.y, 1.0, BLACK);
                }
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
    request_new_screen_size(SCREEN_WIDTH, SCREEN_HEIGHT);

    let scale = screen_height() / UNITS;
    let mut gs = GameState {
        run_state: RunState::Running,
        scl: scale,
        player: Spaceship {
            w: PLAYER_WIDTH,
            h: PLAYER_HEIGHT,
            pos: vec2(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0),
            angle: 0.0,
            vel: vec2(0.0, 0.0),
            last_turret_frame: 0.0,
        },
        asteroids: vec![
            Asteroid {
                pos: vec2(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0),
                // vel: vec2(ASTEROID_VELOCITY, ASTEROID_VELOCITY),
                vel: vec2(0.0, 0.0),
                size: 3,
                points: get_points(
                    vec2(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0),
                    8,
                    3.0 * scale / 2.0
                )
            };
            6
        ],
        bullets: Vec::new(),
        lives: 5,
    };

    loop {
        gs.scl = screen_height() / UNITS;

        handle_input(&mut gs);
        update(&mut gs);
        draw(&gs);

        next_frame().await
    }
}
