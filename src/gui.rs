use super::{
    draw_spaceship, GameState, Spaceship, FONT_SIZE, GAME_TIME, MAX_PLAYER_LIVES, PLAYER_HEIGHT,
    PLAYER_WIDTH,
};
use macroquad::{
    color_u8,
    prelude::{
        draw_rectangle, draw_text, measure_text, screen_height, screen_width, vec2, Color, BLACK,
        GRAY, RED, WHITE,
    },
};

pub const GUI_BAR_HEIGHT: f32 = 50.0;
pub const GUI_NUMBER_FONT_SIZE: f32 = 60.0;

pub fn draw(gs: &GameState) {
    draw_rectangle(
        0.0,
        screen_height() - GUI_BAR_HEIGHT,
        screen_width(),
        GUI_BAR_HEIGHT,
        BLACK,
    );

    // draw score
    let bg_score_string = &"00000".to_string();
    let bg_text_size = measure_text(bg_score_string, None, GUI_NUMBER_FONT_SIZE as _, 1.0);
    let score_string = &gs.score.to_string();
    let text_size = measure_text(score_string, None, GUI_NUMBER_FONT_SIZE as _, 1.0);
    draw_text(
        bg_score_string,
        screen_width() - bg_text_size.width - 10.0,
        screen_height() - GUI_BAR_HEIGHT / 2.0 + bg_text_size.height / 2.0,
        GUI_NUMBER_FONT_SIZE,
        GRAY,
    );
    draw_rectangle(
        screen_width() - text_size.width - 10.0,
        screen_height() - GUI_BAR_HEIGHT,
        text_size.width,
        GUI_BAR_HEIGHT,
        BLACK,
    );
    draw_text(
        &score_string,
        screen_width() - text_size.width - 10.0,
        screen_height() - GUI_BAR_HEIGHT / 2.0 + text_size.height / 2.0,
        GUI_NUMBER_FONT_SIZE,
        WHITE,
    );

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
