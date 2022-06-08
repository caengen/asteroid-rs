use macroquad::prelude::*;
use std::ops::Add;

const DEBUG: bool = true;

const SCREEN_WIDTH: f32 = 600.0;
const SCREEN_HEIGHT: f32 = 400.0;
const ASTEROID_MAX_SIZE: f32 = 3.0;
const ASTEROID_VEL: f32 = 60.0;
const FRICT: f32 = 0.75;
// const UNITS: f32 = 32.0;
const UNITS: f32 = 16.0;
const MAX_PLAYER_LIVES: i8 = 3;
const PLAYER_WIDTH: f32 = 1.0;
const PLAYER_HEIGHT: f32 = 1.0;
const PLAYER_ACCL: f32 = 5.0;
const ANGLE_STEP: f32 = 5.0;
const BULLET_WIDTH: f32 = 4.0;
const BULLET_VEL: f32 = 300.0;
const BULLET_LIVE_TIME: f64 = 1.0; // in seconds
const TURRET_COOLDOWN: f64 = 0.5; // in seconds

enum RunState {
    Menu,
    Running,
    GameOver,
}

#[derive(Clone)]
struct Asteroid {
    pos: Vec2,
    vel: Vec2,
    angle: f32,
    size: f32,
    points: Vec<Vec2>,
    w: f32,
    collision: bool,
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

struct Bullet {
    pos: Vec2,
    created_at: f64,
    vel: Vec2,
    collision: bool,
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

/*
    Line segment intersection algorithm.
    The lines AB and CD intersect if and only if points A and B are separated
    by segment CD and points C and D are separated by segment AB. If points
    A and B are separated by segment CD then ACD and BCD should have opposite
    orientation meaning either ACD or BCD is counterclockwise but not both.
    ref: https://bryceboe.com/2006/10/23/line-segment-intersection-algorithm/
*/
fn ccw(a: Vec2, b: Vec2, c: Vec2) -> bool {
    (c.y - a.y) * (b.x - a.x) > (b.y - a.y) * (c.x - a.x)
}

fn intersects(a: Vec2, b: Vec2, c: Vec2, d: Vec2) -> bool {
    ccw(a, c, d) != ccw(b, c, d) && ccw(a, b, c) != ccw(a, b, d)
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
                    collision: false,
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

fn create_polygon_points(origo: Vec2, amount: i32, size: f32) -> Vec<Vec2> {
    let mut points = Vec::new();
    let angle_inc = 360.0 / amount as f32;
    for i in 1..=amount {
        let rot = (angle_inc * i as f32).to_radians();
        let r = crate::rand::gen_range(0.75, 1.0);
        points.push(vec2(
            origo.x + PLAYER_WIDTH * r * size * rot.sin(),
            origo.y - PLAYER_WIDTH * r * size * rot.cos(),
        ));
    }

    points
}

fn spawn_asteroids(spawn_point: Vec2, r: f32, amount: i32, size: f32, scl: f32) -> Vec<Asteroid> {
    let mut asteroids = Vec::new();
    let angle_inc = 360.0 / amount as f32;

    for i in 1..=amount {
        let rot = ((angle_inc * i as f32 + (30.0 * crate::rand::gen_range(0.1, 1.0))) % 360.0)
            .to_radians();
        let rand_vel_x = ASTEROID_VEL / size;
        let rand_vel_y = ASTEROID_VEL / size;
        let pos = vec2(spawn_point.x + r * rot.sin(), spawn_point.y - r * rot.cos());
        let points = create_polygon_points(vec2(0.0, 0.0), 8, size * scl);
        let w = points[0].distance(points[(points.len() / 2) as usize]);
        let a = Asteroid {
            pos,
            vel: vec2(rand_vel_x, rand_vel_y),
            size,
            points,
            w,
            angle: rot,
            collision: false,
        };
        asteroids.push(a)
    }

    asteroids
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

            // update asteroids
            for asteroid in gs.asteroids.iter_mut() {
                asteroid.pos = wrap(
                    asteroid.pos + (asteroid.vel * delta),
                    asteroid.w,
                    asteroid.w,
                );
                asteroid.angle = (asteroid.angle + 1.5 / asteroid.size) % 360.0;
            }

            // update bullets
            for bullet in gs.bullets.iter_mut() {
                let a = bullet.pos;
                bullet.pos = wrap(
                    bullet.pos + (bullet.vel * delta),
                    BULLET_WIDTH,
                    BULLET_WIDTH,
                );
                let b = bullet.pos;

                // check for collisions
                for ast in gs.asteroids.iter_mut() {
                    let p = ast.get_points();
                    for i in 0..p.len() {
                        if intersects(a, b, p[i], p[(i + 1) % p.len()]) {
                            bullet.collision = true;
                            break;
                        }
                    }

                    if bullet.collision {
                        ast.collision = true;
                        break;
                    }
                }
            }
            gs.bullets
                .retain(|b| time - b.created_at < BULLET_LIVE_TIME && !b.collision);

            let mut new_asteroids = Vec::new();
            gs.asteroids.retain(|a| {
                if (a.collision && a.size > 1.0) {
                    new_asteroids.append(&mut spawn_asteroids(
                        a.pos,
                        a.w / 4.0,
                        a.size as i32,
                        a.size - 1.0,
                        gs.scl,
                    ));
                }

                !a.collision
            });
            if new_asteroids.len() > 0 {
                gs.asteroids.append(&mut new_asteroids);
            }

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

    let rot = angle.to_radians();
    let sh = h * scl; // ship height
    let sw = w * scl; // ship width

    let v1 = vec2(
        ship.pos.x + rot.sin() * sh / 2.,
        ship.pos.y - rot.cos() * sh / 2.,
    );
    let v2 = vec2(
        ship.pos.x - rot.cos() * sw / 2. - rot.sin() * sh / 2.,
        ship.pos.y - rot.sin() * sw / 2. + rot.cos() * sh / 2.,
    );
    let v3 = vec2(
        ship.pos.x + rot.cos() * sw / 2. - rot.sin() * sh / 2.,
        ship.pos.y + rot.sin() * sw / 2. + rot.cos() * sh / 2.,
    );
    let v4 = vec2(
        ship.pos.x - rot.cos() * sw / 1.5 - rot.sin() * sh / 1.5,
        ship.pos.y - rot.sin() * sw / 1.5 + rot.cos() * sh / 1.5,
    );
    let v5 = vec2(
        ship.pos.x + rot.cos() * sw / 1.5 - rot.sin() * sh / 1.5,
        ship.pos.y + rot.sin() * sw / 1.5 + rot.cos() * sh / 1.5,
    );

    draw_triangle_lines(v1, v2, v3, 1.0, BLACK);
    draw_line(v2.x, v2.y, v4.x, v4.y, 1.0, BLACK);
    draw_line(v3.x, v3.y, v5.x, v5.y, 1.0, BLACK);

    // draw_circle(pos.x, pos.y, 0.1 * scl, RED);

    if DEBUG {
        draw_line(
            pos.x,
            pos.y,
            pos.x + vel.x * scl / 2.0,
            pos.y + vel.y * scl / 2.0,
            1.0,
            GREEN,
        );
        if vel.x != 0.0 && vel.y != 0.0 {
            draw_text(
                &format!("Vel: {}", vel.to_string()),
                pos.x + vel.x * scl / 2.0 + 5.0,
                pos.y + vel.y * scl / 2.0 + 5.0,
                15.0,
                GREEN,
            );
        }
    }
}

fn draw_ui(gs: &GameState) {
    draw_text(
        "LIVES",
        screen_width() - (PLAYER_WIDTH * gs.scl) * MAX_PLAYER_LIVES as f32,
        20.0,
        20.0,
        BLACK,
    );
    let mut mock = Spaceship {
        w: PLAYER_WIDTH / 2.0,
        h: PLAYER_HEIGHT / 2.0,
        pos: vec2(0.0, 0.0),
        angle: 0.0,
        vel: vec2(0.0, 0.0),
        last_turret_frame: 0.0,
    };
    for i in 0..gs.lives {
        mock.pos = vec2(
            screen_width() - (PLAYER_WIDTH * gs.scl) * (gs.lives - i) as f32,
            35.0,
        );
        draw_spaceship(&mock, gs.scl)
    }
}

fn draw(gs: &GameState) {
    clear_background(WHITE);

    match gs.run_state {
        RunState::Running => {
            draw_spaceship(&gs.player, gs.scl);

            for bullet in gs.bullets.iter() {
                draw_circle(bullet.pos.x, bullet.pos.y, BULLET_WIDTH / 2.0, BLACK)
            }

            for asteroid in gs.asteroids.iter() {
                let p = asteroid.get_points();
                for i in 0..=(p.len() - 1) {
                    let p1 = p[i];
                    let p2 = p[(i + 1) % p.len()];
                    draw_line(p1.x, p1.y, p2.x, p2.y, 1.0, BLACK);
                }
            }

            draw_ui(&gs);

            if DEBUG {
                draw_text(&format!("fps: {}", get_fps()), 10.0, 50.0, 15.0, BLACK);
                draw_text(
                    &format!("Vel: {}", gs.player.vel.to_string()),
                    10.0,
                    60.0,
                    15.0,
                    BLACK,
                );
                draw_text(
                    &format!("Angle: {}", gs.player.angle.to_string()),
                    10.0,
                    70.0,
                    15.0,
                    BLACK,
                );
                draw_text(
                    &format!("W:{}, H:{}", screen_width(), screen_height()),
                    10.0,
                    80.0,
                    15.0,
                    BLACK,
                );
                draw_text(
                    &format!("Asteroid count: {}", gs.asteroids.len()),
                    10.0,
                    90.0,
                    15.0,
                    BLACK,
                );
            }
        }
        _ => {}
    }
}

#[macroquad::main("asteroids.rs")]
async fn main() {
    request_new_screen_size(SCREEN_WIDTH, SCREEN_HEIGHT);

    let scale = screen_height() / UNITS;
    let center_pos = vec2(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0);
    let mut gs = GameState {
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
        asteroids: spawn_asteroids(
            center_pos,
            screen_width() / ASTEROID_MAX_SIZE,
            3,
            ASTEROID_MAX_SIZE,
            scale,
        ),
        bullets: Vec::new(),
        lives: MAX_PLAYER_LIVES,
    };

    loop {
        gs.scl = screen_height() / UNITS;

        handle_input(&mut gs);
        update(&mut gs);
        draw(&gs);

        next_frame().await
    }
}
