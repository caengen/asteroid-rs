use std::ops::Add;

use crate::components::PLAYER_WIDTH;

use super::{
    get_new_game_state, Bullet, Exhaust, GameState, RunState, ANGLE_STEP, BULLET_VEL,
    EXHAUST_COOLDOWN, PLAYER_ACCL, TURRET_COOLDOWN,
};
use macroquad::prelude::{
    get_frame_time, get_time, is_key_down, is_key_pressed, rand, vec2, KeyCode,
};

pub fn handle_input(gs: &mut GameState) {
    let delta = get_frame_time();
    let rotation = gs.player.angle.to_radians();
    let sh = gs.player.h * gs.scl; // ship height
    let time = get_time();

    match gs.run_state {
        RunState::Running => {
            if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
                gs.player.angle = (gs.player.angle - ANGLE_STEP) % 360.0;
            }
            if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
                gs.player.angle = (gs.player.angle + ANGLE_STEP) % 360.0;
            }

            if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
                gs.player.vel = vec2(
                    gs.player.vel.x + (PLAYER_ACCL * delta) * rotation.sin(),
                    gs.player.vel.y - (PLAYER_ACCL * delta) * rotation.cos(),
                );
                if time - gs.player.last_exhaust_frame > EXHAUST_COOLDOWN {
                    let mut factor;
                    let mut s;
                    let mut diff;
                    for _i in 0..3 {
                        factor = rand::gen_range(0.3, 1.0);
                        s = rand::gen_range(0.1, 1.);
                        diff = if rand::gen_range(0, 100) < 50 {
                            vec2(-(rotation.cos() * sh / 4.0), -(rotation.sin() * sh / 4.0))
                        } else {
                            vec2(rotation.cos() * sh / 4.0, rotation.sin() * sh / 4.0)
                        };
                        let pos = vec2(
                            gs.player.pos.x - (sh / 2.0 + (sh / 3.0) * factor) * rotation.sin(),
                            gs.player.pos.y + (sh / 2.0 + (sh / 3.0) * factor) * rotation.cos(),
                        );
                        gs.exhaust.push(Exhaust {
                            created_at: time,
                            pos: pos.add(diff * factor),
                            size: 0.5 * s,
                            vel: vec2(
                                -(BULLET_VEL / 6.0 * rotation.sin()),
                                BULLET_VEL / 6.0 * rotation.cos(),
                            )
                            .add(diff),
                        });
                    }
                    gs.player.last_exhaust_frame = time;
                }
            }
            if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
                gs.player.vel = vec2(
                    gs.player.vel.x - PLAYER_ACCL * delta * rotation.sin(),
                    gs.player.vel.y + PLAYER_ACCL * delta * rotation.cos(),
                );
            }
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

            if is_key_pressed(KeyCode::G) {
                gs.debug = !gs.debug;
            }
        }
        RunState::Death => {
            if is_key_down(KeyCode::Space) {
                gs.run_state = RunState::Running;
            }
        }
        RunState::GameOver => {
            if is_key_down(KeyCode::Enter) {
                *gs = get_new_game_state();
            }
        }
        _ => {}
    }
}
