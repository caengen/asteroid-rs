use super::{
    GameState, RunState, Spaceship, BULLET_WIDTH, FONT_SIZE, GAME_TIME, MAX_PLAYER_LIVES,
    PLAYER_HEIGHT, PLAYER_WIDTH,
};
use macroquad::prelude::{
    clear_background, draw_circle, draw_line, draw_text, draw_triangle, get_fps, measure_text,
    screen_height, screen_width, vec2, BLACK, GREEN, WHITE,
};

pub fn draw_spaceship(ship: &Spaceship, scl: f32, debug: bool) {
    let Spaceship { pos, vel, .. } = ship;

    let p = ship.get_points(scl);

    draw_triangle(p[0], p[1], p[2], WHITE);
    draw_line(p[1].x, p[1].y, p[3].x, p[3].y, 1.0, WHITE);
    draw_line(p[2].x, p[2].y, p[4].x, p[4].y, 1.0, WHITE);

    // draw_circle(pos.x, pos.y, 0.1 * scl, RED);

    if debug {
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

pub fn draw_ui(gs: &GameState) {
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
            screen_width() - PLAYER_WIDTH * gs.scl * MAX_PLAYER_LIVES as f32
                + (PLAYER_WIDTH * gs.scl * i as f32),
            35.0,
        );
        draw_spaceship(&mock, gs.scl, gs.debug)
    }
}

pub fn draw(gs: &GameState) {
    clear_background(BLACK);

    match gs.run_state {
        RunState::Running | RunState::Death => {
            draw_spaceship(&gs.player, gs.scl, gs.debug);

            for bullet in gs.bullets.iter() {
                draw_circle(
                    bullet.pos.x,
                    bullet.pos.y,
                    BULLET_WIDTH / 2.0 * gs.scl,
                    WHITE,
                )
            }

            for asteroid in gs.asteroids.iter() {
                let p = asteroid.get_points();
                for i in 0..=(p.len() - 1) {
                    let p1 = p[i];
                    let p2 = p[(i + 1) % p.len()];
                    draw_line(p1.x, p1.y, p2.x, p2.y, 1.0, WHITE);
                }
            }

            draw_ui(&gs);

            if gs.run_state == RunState::Death {
                let text = "Press Space to start.";
                let text_size = measure_text(text, None, FONT_SIZE as _, 1.0);
                draw_text(
                    text,
                    screen_width() / 2.0 - text_size.width / 2.0,
                    screen_height() / 2.0 + PLAYER_HEIGHT * 2.0 * gs.scl,
                    FONT_SIZE,
                    WHITE,
                );
            }

            if gs.debug {
                draw_text(
                    &format!("fps: {}", get_fps()),
                    10.0,
                    50.0,
                    FONT_SIZE - 5.0,
                    WHITE,
                );
                draw_text(
                    &format!("Vel: {}", gs.player.vel.to_string()),
                    10.0,
                    60.0,
                    FONT_SIZE - 5.0,
                    WHITE,
                );
                draw_text(
                    &format!("Angle: {}", gs.player.angle.to_string()),
                    10.0,
                    70.0,
                    FONT_SIZE - 5.0,
                    WHITE,
                );
                draw_text(
                    &format!("W:{}, H:{}", screen_width(), screen_height()),
                    10.0,
                    80.0,
                    FONT_SIZE - 5.0,
                    WHITE,
                );
                draw_text(
                    &format!("Player lives: {}", gs.lives),
                    10.0,
                    90.0,
                    FONT_SIZE - 5.0,
                    WHITE,
                );
                draw_text(
                    &format!("Asteroid count: {}", gs.asteroids.len()),
                    10.0,
                    100.0,
                    FONT_SIZE - 5.0,
                    WHITE,
                );
            }
        }
        RunState::GameOver => {
            let sw = screen_width();
            let sh = screen_height();
            let size = FONT_SIZE * 1.5;
            let text = "Game over.";
            let text_size = measure_text(text, None, size as _, 1.0);
            draw_text(
                text,
                sw / 2.0 - text_size.width / 2.0,
                sh / 4.0,
                size,
                WHITE,
            );
            draw_text(
                format!("Life multiplier x{}", gs.lives + 1).as_str(),
                sw / 2.0 - 60.0,
                sh / 4.0 + 20.0,
                FONT_SIZE,
                WHITE,
            );
            let timex = ((GAME_TIME - gs.play_time) / 10.0) as i32;
            draw_text(
                format!("Time multiplier x{}", timex as i32).as_str(),
                sw / 2.0 - 60.0,
                sh / 4.0 + 40.0,
                FONT_SIZE,
                WHITE,
            );
            draw_text(
                format!("Final score: {}", (gs.score * (gs.lives + 1)) * timex).as_str(),
                sw / 2.0 - 60.0,
                sh / 4.0 + 60.0,
                FONT_SIZE,
                WHITE,
            );

            let text = "Press Enter to restart.";
            let text_size = measure_text(text, None, FONT_SIZE as _, 1.0);
            draw_text(
                text,
                sw / 2.0 - text_size.width / 2.0,
                sh / 4.0 + 80.0,
                FONT_SIZE,
                WHITE,
            )
        }
        _ => {}
    }
}
