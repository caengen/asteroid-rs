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

struct Spaceship {
    w: f32,
    h: f32,
    pos: Vec2,
    angle: f32,
    vel: Vec2,
    acc: f32,
}
struct GameState {
    scl: f32, // scale
    player: Spaceship,
    asteroids: Vec<Asteroid>,
    lives: i8,
    run_state: RunState,
}

fn handle_input(gs: &mut GameState) {
    let delta = get_frame_time();
    match gs.run_state {
        RunState::Running => {
            if is_key_down(KeyCode::Left) {
                gs.player.angle -= 5.0;
            }
            if is_key_down(KeyCode::Right) {
                gs.player.angle += 5.0;
            }

            let rotation = gs.player.angle.to_radians();
            if is_key_down(KeyCode::Up) {
                gs.player.vel = vec2(
                    gs.player.vel.x + (7.0 * delta) * rotation.sin(),
                    gs.player.vel.y + (7.0 * delta) * rotation.cos(),
                );
            }
            if is_key_down(KeyCode::Down) {
                gs.player.vel = vec2(
                    gs.player.vel.x - (7.0 * delta) * rotation.sin(),
                    gs.player.vel.y - (7.0 * delta) * rotation.cos(),
                );
            }
        }
        _ => {}
    }
}

fn update(gs: &mut GameState) {
    let delta = get_frame_time();
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

            // handle bounds
            if gs.player.pos.x > screen_width() {
                gs.player.pos.x = 0.0 - gs.player.w;
            } else if gs.player.pos.x < 0.0 - gs.player.w {
                gs.player.pos.x = screen_width();
            }

            if gs.player.pos.y > screen_height() {
                gs.player.pos.y = 0.0 - gs.player.h;
            } else if gs.player.pos.y < 0.0 - gs.player.h {
                gs.player.pos.y = screen_height();
            }
        }
        _ => {}
    }
}

fn rotate(point: Vec2, angle: &f32, origin: &Vec2) -> Vec2 {
    let rotation = angle.to_radians();
    let cos = rotation.cos();
    let sin = rotation.sin();
    let mut p = point;
    p.x -= origin.x;
    p.y -= origin.y;
    let new_x = p.x * cos - p.y * sin;
    let new_y = p.x * sin - p.y * cos;

    p.x = new_x + origin.x;
    p.y = new_y + origin.y;

    p
}

// let A = vec2(pos.x, pos.y + (h * scl / 2.0));
// let B = vec2(pos.x - (w * scl / 2.0), pos.y - (h * scl / 2.0));
// let C = vec2(pos.x + (w * scl / 2.0), pos.y - (h * scl / 2.0));
// let D = vec2(pos.x - ((w + w / 2.0) * scl / 2.0), pos.y - (h * scl));
// let E = vec2(pos.x + ((w + w / 2.0) * scl / 2.0), pos.y - (h * scl));

// let centroid = vec2((A.x + D.x + E.x) / 3.0, (A.y - E.y) / 3.0 + E.y);
// let Ar = rotate(A, angle, &pos);
// let Br = rotate(B, angle, &pos);
// let Cr = rotate(C, angle, &pos);
// let Dr = rotate(D, angle, &pos);
// let Er = rotate(E, angle, &pos);
fn draw_spaceship(ship: &Spaceship, scl: f32) {
    let Spaceship {
        pos, angle, w, h, ..
    } = ship;

    // let pd = 10.0;
    // let A = rotate(vec2(0.0, pd), angle, &vec2(0.0, 0.0));
    // let B = rotate(vec2(-pd, -pd), angle, &vec2(0.0, 0.0));
    // let C = rotate(vec2(pd, -pd), angle, &vec2(0.0, 0.0));

    let rotation = angle.to_radians();
    let SHIP_HEIGHT = h * scl;
    let SHIP_BASE = w * scl;
    // let new_x = p.x * cos - p.y * sin;
    // let new_y = p.x * sin - p.y * cos;

    let v1 = Vec2::new(
        ship.pos.x + rotation.sin() * SHIP_HEIGHT / 2., // hvorfor ikke minus delen her
        ship.pos.y - rotation.cos() * SHIP_HEIGHT / 2.,
    );
    let v2 = Vec2::new(
        ship.pos.x - rotation.cos() * SHIP_BASE / 2. - rotation.sin() * SHIP_HEIGHT / 2.,
        ship.pos.y - rotation.sin() * SHIP_BASE / 2. + rotation.cos() * SHIP_HEIGHT / 2.,
    );
    let v3 = Vec2::new(
        ship.pos.x + rotation.cos() * SHIP_BASE / 2. - rotation.sin() * SHIP_HEIGHT / 2.,
        ship.pos.y + rotation.sin() * SHIP_BASE / 2. + rotation.cos() * SHIP_HEIGHT / 2.,
    );

    draw_triangle_lines(v1, v2, v3, 1.0, BLACK);

    // draw_line(Br.x, Br.y, Dr.x, Dr.y, 1.0, BLACK);
    // draw_line(Cr.x, Cr.y, Er.x, Er.y, 1.0, BLACK);

    // draw_circle(centroid.x, centroid.y, 0.1 * scl, GREEN);
    draw_circle(pos.x, pos.y, 0.1 * scl, RED);
    // draw_circle(centroid.x, centroid.y, 0.1 * scl, RED)
}

fn draw(gs: &GameState) {
    clear_background(WHITE);

    match gs.run_state {
        RunState::Running => {
            draw_spaceship(&gs.player, gs.scl);
            draw_text(
                &format!("{}", gs.player.vel.to_string()),
                100.0,
                100.0,
                30.0,
                BLACK,
            )
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
            angle: 90.0,
            vel: vec2(0.0, 0.0),
            acc: 0.0,
        },
        asteroids: vec![
            Asteroid {
                pos: vec2(0.0, 0.0),
                vel: vec2(ASTEROID_VELOCITY, ASTEROID_VELOCITY),
                size: 3
            };
            6
        ],
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
