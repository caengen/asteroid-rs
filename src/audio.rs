use super::GameState;
use macroquad::audio::{set_sound_volume, Sound};
use macroquad::{
    audio::{load_sound, play_sound, PlaySoundParams},
    prelude::*,
};

#[derive(Clone)]
pub enum GameSound {
    Shot = 0,
    ExplosionLarge = 1,
    ExplosionMedium = 2,
    ExplosionSmall = 3,
    Death = 4,
}

#[derive(Clone)]
pub struct GameSoundDictEntry {
    pub game_sound: GameSound,
    pub filepath: String,
}

pub async fn load_assets(gs: &mut GameState) {
    let mut files = vec![
        GameSoundDictEntry {
            game_sound: GameSound::Shot,
            filepath: "assets/audio/shot.wav".to_string(),
        },
        GameSoundDictEntry {
            game_sound: GameSound::ExplosionLarge,
            filepath: "assets/audio/explosion3.wav".to_string(),
        },
        GameSoundDictEntry {
            game_sound: GameSound::ExplosionMedium,
            filepath: "assets/audio/explosion2.wav".to_string(),
        },
        GameSoundDictEntry {
            game_sound: GameSound::ExplosionSmall,
            filepath: "assets/audio/explosion1.wav".to_string(),
        },
    ];

    for file in files.iter() {
        let s = load_sound(file.filepath.as_str()).await;
        let i = file.clone().game_sound as usize; // clone fixes shared ref error...
        match s {
            Err(e) => gs.sounds[i] = None,
            Ok(val) => gs.sounds[i] = Some(val),
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
