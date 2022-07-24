use super::GameState;
use macroquad::audio::{set_sound_volume, Sound};
use macroquad::prelude::state_machine::StateMachine::Ready;
use macroquad::{
    audio::{load_sound, play_sound, PlaySoundParams},
    prelude::*,
};
use std::future::Pending;

pub enum GameSound {
    Shot = 0,
    ExplosionLarge = 1,
    ExplosionMedium = 2,
    ExplosionSmall = 3,
}

pub async fn load_assets(gs: &mut GameState) {
    let mut sounds = Vec::new();
    sounds.push((GameSound::Shot, load_sound("assets/audio/shot.wav").await));
    sounds.push((
        GameSound::ExplosionLarge,
        load_sound("assets/audio/explosion-large.wav").await,
    ));
    sounds.push((
        GameSound::ExplosionMedium,
        load_sound("assets/audio/explosion-medium.wav").await,
    ));
    sounds.push((
        GameSound::ExplosionSmall,
        load_sound("assets/audio/explosion-small.wav").await,
    ));

    for (sound, res) in sounds {
        match res {
            Err(e) => {}
            Ok(val) => gs.sounds[sound as usize] = Some(val),
        }
    }
}

pub fn play_audio(sounds: &Vec<Option<Sound>>, sound: GameSound) {
    let s = sounds[sound as usize];

    match s {
        None => {}
        Some(s) => play_sound(
            s,
            PlaySoundParams {
                looped: false,
                volume: 0.5,
            },
        ),
    }
}
