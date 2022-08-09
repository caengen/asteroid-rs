use super::{
    gui, GameState, RunState, Spaceship, ASTEROID_MAX_SIZE, BULLET_WIDTH, DARK,
    EXPLOSION_LIVE_TIME, FONT_SIZE, GAME_TIME, LIGHT, PLAYER_HEIGHT, POINT_FONT_SIZE,
};
use macroquad::prelude::{
    clear_background, draw_circle, draw_line, draw_rectangle_lines, draw_text, draw_triangle,
    get_fps, get_time, measure_text, rand, screen_height, screen_width, Color, BLACK, GRAY, GREEN,
    LIGHTGRAY, RED,
};

pub fn draw_spaceship(ship: &Spaceship, scl: f32, debug: bool) {
    let Spaceship {
        angle,
        pos,
        vel,
        strafing,
        ..
    } = ship;

    let p = ship.points(scl);

    draw_triangle(p[0], p[1], p[2], LIGHT);
    draw_line(p[1].x, p[1].y, p[3].x, p[3].y, 2.0, LIGHT);
    draw_line(p[2].x, p[2].y, p[4].x, p[4].y, 2.0, LIGHT);

    let (left_strafe, right_strafe) = strafing;
    let rot = angle.to_radians();
    let rand_len = rand::gen_range(0.0, ship.w * 0.8);
    if *left_strafe {
        let x = pos.x - ship.w / 2.0 * scl * rot.cos();
        let y = pos.y - ship.w / 2.0 * scl * rot.sin();
        draw_line(
            x,
            y,
            x - rand_len * scl * rot.cos(),
            y - rand_len * scl * rot.sin(),
            1.75,
            LIGHTGRAY,
        );
    } else if *right_strafe {
        let x = pos.x + ship.w / 2.0 * scl * rot.cos();
        let y = pos.y + ship.w / 2.0 * scl * rot.sin();
        draw_line(
            x,
            y,
            x + rand_len * scl * rot.cos(),
            y + rand_len * scl * rot.sin(),
            1.75,
            LIGHTGRAY,
        );
    }

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

fn draw_background(gs: &GameState) {
    for star in gs.background.iter() {
        draw_circle(star.pos.x, star.pos.y, star.size, LIGHTGRAY);
    }
}

fn draw_debug(gs: &GameState) {
    draw_text(
        &format!("fps: {}", get_fps()),
        10.0,
        50.0,
        FONT_SIZE - 5.0,
        LIGHT,
    );
    draw_text(
        &format!("Vel: {}", gs.player.vel.to_string()),
        10.0,
        60.0,
        FONT_SIZE - 5.0,
        LIGHT,
    );
    draw_text(
        &format!("Angle: {}", gs.player.angle.to_string()),
        10.0,
        70.0,
        FONT_SIZE - 5.0,
        LIGHT,
    );
    draw_text(
        &format!("W:{}, H:{}", screen_width(), screen_height()),
        10.0,
        80.0,
        FONT_SIZE - 5.0,
        LIGHT,
    );
    draw_text(
        &format!("Player lives: {}", gs.lives),
        10.0,
        90.0,
        FONT_SIZE - 5.0,
        LIGHT,
    );
    draw_text(
        &format!("Asteroid count: {}", gs.asteroids.len()),
        10.0,
        100.0,
        FONT_SIZE - 5.0,
        LIGHT,
    );
    draw_text(
        &format!("Exhaust count: {}", gs.exhaust.len()),
        10.0,
        110.0,
        FONT_SIZE - 5.0,
        LIGHT,
    );
}

pub fn draw(gs: &GameState) {
    clear_background(DARK);
    draw_background(gs);

    match gs.run_state {
        RunState::Running | RunState::Death | RunState::StageComplete => {
            draw_spaceship(&gs.player, gs.scl, gs.debug);

            for e in gs.explosions.iter() {
                let thickness = 150.0 * e.size / ASTEROID_MAX_SIZE;
                draw_rectangle_lines(
                    e.pos.x,
                    e.pos.y,
                    e.width,
                    e.width,
                    thickness
                        - thickness * ((get_time() - e.created_at) / EXPLOSION_LIVE_TIME) as f32,
                    LIGHT,
                );
            }

            for bullet in gs.bullets.iter() {
                draw_circle(
                    bullet.pos.x,
                    bullet.pos.y,
                    BULLET_WIDTH / 2.0 * gs.scl,
                    LIGHT,
                )
            }

            for ex in gs.exhaust.iter() {
                draw_line(
                    ex.pos.x - (ex.size / 2.0) * gs.scl,
                    ex.pos.y,
                    ex.pos.x + (ex.size / 2.0) * gs.scl,
                    ex.pos.y,
                    2.0,
                    LIGHT,
                );
                draw_line(
                    ex.pos.x,
                    ex.pos.y - (ex.size / 2.0) * gs.scl,
                    ex.pos.x,
                    ex.pos.y + (ex.size / 2.0) * gs.scl,
                    2.0,
                    LIGHT,
                );
            }

            for asteroid in gs.asteroids.iter() {
                let p = asteroid.points();
                for i in 0..=(p.len() - 1) {
                    let p1 = p[i];
                    let p2 = p[(i + 1) % p.len()];
                    // bug: not drawing over star background..
                    draw_triangle(p1, p2, asteroid.pos, DARK);
                    draw_line(p1.x, p1.y, p2.x, p2.y, 2.0, LIGHT);
                }
            }

            for point in gs.flying_points.iter() {
                let text = &format!("{}", point.val);
                let text_measure = measure_text(text, None, POINT_FONT_SIZE as _, 1.0);
                draw_text(
                    text,
                    point.pos.x - text_measure.width / 2.0,
                    point.pos.y - text_measure.height / 2.0,
                    POINT_FONT_SIZE,
                    LIGHT,
                );
            }

            gui::draw(&gs);

            if gs.run_state == RunState::Death {
                let text = "Press Space to start.";
                let text_size = measure_text(text, None, FONT_SIZE as _, 1.0);
                draw_text(
                    text,
                    screen_width() / 2.0 - text_size.width / 2.0,
                    screen_height() / 2.0 + PLAYER_HEIGHT * 2.0 * gs.scl,
                    FONT_SIZE,
                    LIGHT,
                );
            }

            if gs.debug {
                draw_debug(gs);
            }
        }
        RunState::StageComplete => {
            // let sw = screen_width();
            // let sh = screen_height();
            // let size = FONT_SIZE * 1.5;
            // let text = "Game over.";
            // let text_size = measure_text(text, None, size as _, 1.0);
            // draw_text(
            //     text,
            //     sw / 2.0 - text_size.width / 2.0,
            //     sh / 4.0,
            //     size,
            //     LIGHT,
            // );
            // draw_text(
            //     format!("Life multiplier x{}", gs.lives + 1).as_str(),
            //     sw / 2.0 - 60.0,
            //     sh / 4.0 + 20.0,
            //     FONT_SIZE,
            //     LIGHT,
            // );
            // let timex = ((GAME_TIME - gs.play_time) / 10.0) as i32;
            // draw_text(
            //     format!("Time multiplier x{}", timex as i32).as_str(),
            //     sw / 2.0 - 60.0,
            //     sh / 4.0 + 40.0,
            //     FONT_SIZE,
            //     LIGHT,
            // );
            // draw_text(
            //     format!("Final score: {}", (gs.score * (gs.lives + 1)) * timex).as_str(),
            //     sw / 2.0 - 60.0,
            //     sh / 4.0 + 60.0,
            //     FONT_SIZE,
            //     LIGHT,
            // );

            // let text = "Press Enter to restart.";
            // let text_size = measure_text(text, None, FONT_SIZE as _, 1.0);
            // draw_text(
            //     text,
            //     sw / 2.0 - text_size.width / 2.0,
            //     sh / 4.0 + 80.0,
            //     FONT_SIZE,
            //     LIGHT,
            // )
        }
        _ => {}
    }
}
