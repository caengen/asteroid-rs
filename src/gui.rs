use super::{
    draw_spaceship, GameState, Spaceship, FONT_SIZE, GAME_TIME, MAX_PLAYER_LIVES, PLAYER_HEIGHT,
    PLAYER_WIDTH,
};
use macroquad::prelude::{draw_text, screen_width, vec2, WHITE};

pub fn draw(gs: &GameState) {
    draw_text("SCORE", 20.0, 20.0, FONT_SIZE, WHITE);
    draw_text(&gs.score.to_string(), 20.0, 35.0, FONT_SIZE + 5.0, WHITE);

    draw_text("TIME", screen_width() / 2.0 - 20.0, 20.0, FONT_SIZE, WHITE);
    draw_text(
        &((GAME_TIME - gs.play_time) as i8).to_string(),
        screen_width() / 2.0 - 10.0,
        35.0,
        FONT_SIZE + 5.0,
        WHITE,
    );
    draw_text(
        "LIVES",
        screen_width() - (PLAYER_WIDTH * gs.scl) * MAX_PLAYER_LIVES as f32,
        20.0,
        FONT_SIZE,
        WHITE,
    );
    let mut mock = Spaceship::new(0.0, 0.0, PLAYER_WIDTH / 2., PLAYER_HEIGHT / 2.);
    for i in 0..gs.lives {
        mock.pos = vec2(
            screen_width() - PLAYER_WIDTH * gs.scl * MAX_PLAYER_LIVES as f32
                + (PLAYER_WIDTH * gs.scl * i as f32),
            35.0,
        );
        draw_spaceship(&mock, gs.scl, gs.debug)
    }
}
