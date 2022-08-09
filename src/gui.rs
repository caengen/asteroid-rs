use crate::components::{RunState, COMBO_TIMER};

use super::{
    draw_spaceship, GameState, Spaceship, DARK, GAME_TIME, LIGHT, MAX_PLAYER_LIVES, PLAYER_HEIGHT,
    PLAYER_WIDTH,
};
use macroquad::prelude::{
    draw_rectangle, draw_text, draw_triangle, measure_text, screen_height, screen_width, vec2, GRAY,
};

pub const GUI_BAR_HEIGHT: f32 = 50.0;
pub const GUI_NUMBER_FONT_SIZE: f32 = 50.0;

pub fn draw(gs: &GameState) {
    draw_rectangle(
        0.0,
        screen_height() - GUI_BAR_HEIGHT,
        screen_width(),
        GUI_BAR_HEIGHT,
        DARK,
    );

    if gs.run_state == RunState::StageComplete {
        // skriv stage complete
    }
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
        DARK,
    );
    draw_text(
        &score_string,
        screen_width() - text_size.width - 10.0,
        screen_height() - GUI_BAR_HEIGHT / 2.0 + text_size.height / 2.0,
        GUI_NUMBER_FONT_SIZE,
        LIGHT,
    );

    //draw combo timer
    let th = GUI_BAR_HEIGHT * 0.5;
    let tw = screen_width() / 5.0;
    let tx = GUI_BAR_HEIGHT * 1.5;
    let ty = screen_height() - GUI_BAR_HEIGHT / 2.0 - th / 2.0;
    draw_rectangle(tx, ty, tw, th, GRAY); //bg
    if gs.combo_time > 0.0 {
        draw_rectangle(tx, ty, tw * (gs.combo_time / COMBO_TIMER), th, LIGHT); //actual timer
    }
    draw_triangle(vec2(tx, ty), vec2(tx + 20.0, ty), vec2(tx, ty + th), DARK);
    draw_triangle(
        vec2(tx + tw, ty),
        vec2(tx + 20.0 + tw, ty + th),
        vec2(tx + tw - 20.0, ty + th),
        DARK,
    );

    //draw combo
    let bg_combo_string = &"00".to_string();
    let bg_combo_size = measure_text(bg_combo_string, None, GUI_NUMBER_FONT_SIZE as _, 1.0);
    let combo_string = &gs.combo.to_string();
    let combo_size = measure_text(combo_string, None, GUI_NUMBER_FONT_SIZE as _, 1.0);
    let cx = 85.0;
    let cy = screen_height() - GUI_BAR_HEIGHT / 2.0 - 5.0;
    draw_text(
        bg_combo_string,
        cx - bg_combo_size.width - 10.0,
        cy + bg_combo_size.height / 2.0,
        GUI_NUMBER_FONT_SIZE,
        GRAY,
    );
    if gs.combo > 0 {
        draw_rectangle(
            cx - combo_size.width - 10.0,
            cy - combo_size.height / 2.0,
            combo_size.width,
            combo_size.height,
            DARK,
        );
        draw_text(
            combo_string,
            cx - combo_size.width - 10.0,
            cy + combo_size.height / 2.0,
            GUI_NUMBER_FONT_SIZE,
            LIGHT,
        );
    }

    //draw multiplier
    if gs.score_multiplier > 1 {
        let multiplier = &format!("{}x", gs.score_multiplier).to_string();
        let multiplier_size =
            measure_text(multiplier, None, (GUI_NUMBER_FONT_SIZE - 15.0) as _, 1.0);
        draw_text(
            multiplier,
            tx + tw,
            screen_height() - GUI_BAR_HEIGHT / 2.0 + multiplier_size.height / 2.0,
            GUI_NUMBER_FONT_SIZE - 15.0,
            LIGHT,
        );
    }

    draw_text(
        &((GAME_TIME - gs.play_time) as i8).to_string(),
        screen_width() / 2.0 - 10.0,
        screen_height() - GUI_BAR_HEIGHT / 2.0 + text_size.height / 2.0,
        GUI_NUMBER_FONT_SIZE,
        GRAY,
    );

    let mut mock = Spaceship::new(0.0, 0.0, PLAYER_WIDTH / 2., PLAYER_HEIGHT / 2.);
    for i in 0..gs.lives {
        mock.pos = vec2(
            ((screen_width() / 1.25) + PLAYER_WIDTH * gs.scl)
                - PLAYER_WIDTH * gs.scl * MAX_PLAYER_LIVES as f32
                + (PLAYER_WIDTH * gs.scl * i as f32),
            screen_height() - PLAYER_WIDTH * gs.scl / 1.25,
        );
        draw_spaceship(&mock, gs.scl, gs.debug)
    }
}
