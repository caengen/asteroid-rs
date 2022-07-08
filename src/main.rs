use macroquad::prelude::*;
mod components;
use components::*;
mod draw;
mod gui;
use draw::*;
mod input;
mod spawner;
use input::*;
mod utils;
use utils::*;

fn update(gs: &mut GameState) {
    let delta = get_frame_time();
    let time = get_time();
    match gs.run_state {
        RunState::Running | RunState::Death => {
            if gs.run_state == RunState::Running {
                gs.play_time += delta;
            }

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
            let mut player_collision = false;
            for asteroid in gs.asteroids.iter_mut() {
                asteroid.pos = wrap(
                    asteroid.pos + (asteroid.vel * delta),
                    asteroid.w,
                    asteroid.w,
                );
                asteroid.angle = (asteroid.angle + 1.5 / asteroid.size) % 360.0;

                // check for collisions with player
                if gs.run_state == RunState::Running {
                    let p1 = gs.player.points(gs.scl);
                    let p2 = asteroid.points();
                    for i in 0..3 {
                        let a = p1[i];
                        let b = p1[(i + 1) % 3];
                        for j in 0..p2.len() {
                            if intersects(a, b, p2[j], p2[(j + 1) % p2.len()]) {
                                player_collision = true;
                                break;
                            }
                        }
                        if player_collision {
                            gs.lives -= 1;
                            if gs.lives > 0 {
                                gs.player.reset();
                                gs.run_state = RunState::Death;
                            } else {
                                gs.run_state = RunState::GameOver;
                            }
                            break;
                        }
                    }
                }
            }

            //update particles
            for ex in gs.exhaust.iter_mut() {
                ex.pos += ex.vel * delta;
                ex.size = f32::max(ex.size - 0.5 * delta, 0.0);
            }
            gs.exhaust
                .retain(|e| time - e.created_at < EXHAUST_LIVE_TIME || e.size <= 0.0);

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
                    let p = ast.points();
                    for i in 0..p.len() {
                        if intersects(a, b, p[i], p[(i + 1) % p.len()]) {
                            bullet.collision = true;
                            break;
                        }
                    }

                    if bullet.collision {
                        gs.score += 1 * ast.size as i32;
                        ast.collision = true;
                        break;
                    }
                }
            }
            gs.bullets
                .retain(|b| time - b.created_at < BULLET_LIVE_TIME && !b.collision);

            let mut new_asteroids = Vec::new();
            gs.asteroids.retain(|a| {
                if a.collision {
                    gs.explosions.push(Explosion::new(
                        a.pos.x - a.w / 2.0,
                        a.pos.y - a.w / 2.0,
                        a.w * 0.75,
                        a.size,
                    ));
                    if a.size > 1.0 {
                        new_asteroids.append(&mut spawner::asteroids(
                            a.pos,
                            a.w / 4.0,
                            a.size as i32,
                            a.size - 1.0,
                            gs.scl,
                        ));
                    }
                }

                !a.collision
            });
            if new_asteroids.len() > 0 {
                gs.asteroids.append(&mut new_asteroids);
            }

            gs.explosions
                .retain(|e| time - e.created_at < EXPLOSION_LIVE_TIME);

            if gs.asteroids.len() == 0 {
                gs.run_state = RunState::GameOver;
            }

            // handle player bounds
            gs.player.pos = wrap(gs.player.pos, gs.player.w, gs.player.h)
        }
        _ => {}
    }
}

#[macroquad::main("asteroids.rs")]
async fn main() {
    request_new_screen_size(SCREEN_WIDTH, SCREEN_HEIGHT);
    rand::srand(macroquad::miniquad::date::now() as _);
    let mut gs = get_new_game_state();

    loop {
        gs.scl = screen_height() / UNITS;

        handle_input(&mut gs);
        update(&mut gs);
        draw(&gs);

        next_frame().await
    }
}
