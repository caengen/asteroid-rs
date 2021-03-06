use std::ops::Add;

use super::{
    audio, audio::GameSound, get_new_game_state, spawner, Bullet, Exhaust, GameState, RunState,
    ANGLE_STEP, BULLET_VEL, EXHAUST_COOLDOWN, EXHAUST_VEL, PLAYER_ACCL, PLAYER_WIDTH,
    TURRET_COOLDOWN,
};
use macroquad::{
    audio::{play_sound, PlaySoundParams},
    prelude::{get_frame_time, get_time, is_key_down, is_key_pressed, rand, vec2, KeyCode},
};

pub fn handle_input(gs: &mut GameState) {
    let delta = get_frame_time();
    let rotation = gs.player.angle.to_radians();
    let sh = gs.player.h * gs.scl; // ship height
    let time = get_time();
    gs.player.strafing = (false, false);

    match gs.run_state {
        RunState::Running | RunState::StageComplete => {
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
                spawner::exhaust_particles(gs, EXHAUST_VEL, rotation, sh);
            }
            if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
                gs.player.vel = vec2(
                    gs.player.vel.x - PLAYER_ACCL / 2.0 * delta * rotation.sin(),
                    gs.player.vel.y + PLAYER_ACCL / 2.0 * delta * rotation.cos(),
                );
                spawner::exhaust_particles(gs, -EXHAUST_VEL, rotation, -sh / 4.0);
            }
            if is_key_down(KeyCode::Q) {
                gs.player.vel = vec2(
                    gs.player.vel.x - PLAYER_ACCL / 2.0 * delta * rotation.cos(),
                    gs.player.vel.y - PLAYER_ACCL / 2.0 * delta * rotation.sin(),
                );
                gs.player.strafing = (false, true);
            }
            if is_key_down(KeyCode::E) {
                gs.player.vel = vec2(
                    gs.player.vel.x + PLAYER_ACCL * delta * rotation.cos(),
                    gs.player.vel.y + PLAYER_ACCL * delta * rotation.sin(),
                );
                gs.player.strafing = (true, false);
            }
            if is_key_down(KeyCode::Space) && time - gs.player.last_turret_frame > TURRET_COOLDOWN {
                gs.player.last_turret_frame = time;
                audio::play_audio(&gs.sounds, GameSound::Shot);
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

            if gs.run_state == RunState::StageComplete {
                if is_key_down(KeyCode::Enter) {
                    *gs = get_new_game_state();
                }
            }
        }
        RunState::Death => {
            if is_key_down(KeyCode::Space) {
                gs.run_state = RunState::Running;
            }
        }

        _ => {}
    }
}
