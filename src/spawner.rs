use super::{
    Asteroid, Exhaust, GameState, Star, ASTEROID_VEL, BULLET_VEL, EXHAUST_COOLDOWN, PLAYER_WIDTH,
};
use macroquad::{
    prelude::{get_time, rand, vec2, Vec2},
    rand::{srand, RandomRange},
};
use std::ops::Add;

pub fn polygon(origo: Vec2, amount: i32, size: f32) -> Vec<Vec2> {
    let mut points = Vec::new();
    let angle_inc = 360.0 / amount as f32;

    for i in 1..=amount {
        let rot = (angle_inc * i as f32).to_radians();
        let r = rand::gen_range(0.5, 1.0);
        points.push(vec2(
            origo.x + PLAYER_WIDTH * r * size * rot.sin(),
            origo.y - PLAYER_WIDTH * r * size * rot.cos(),
        ));
    }

    points
}

pub fn asteroids(spawn_point: Vec2, r: f32, amount: i32, size: f32, scl: f32) -> Vec<Asteroid> {
    let mut asteroids = Vec::new();
    let angle_inc = 360.0 / amount as f32;

    for i in 1..=amount {
        let rot =
            ((angle_inc * i as f32 + (30.0 * (rand::gen_range(0.1, 1.0)))) % 360.0).to_radians();
        let pos = vec2(spawn_point.x + r * rot.sin(), spawn_point.y - r * rot.cos());
        let vel = pos * ASTEROID_VEL / 20.0 / size;
        let points = polygon(vec2(0.0, 0.0), 8, size * scl);
        let w = points[0].distance(points[(points.len() / 2) as usize]);
        let a = Asteroid {
            pos,
            vel,
            size,
            points,
            w,
            angle: rot,
            collision: false,
        };
        asteroids.push(a)
    }

    asteroids
}

pub fn exhaust_particles(gs: &mut GameState, vel: f32, rotation: f32, h: f32) {
    let time = get_time();
    if time - gs.player.last_exhaust_frame <= EXHAUST_COOLDOWN {
        return;
    }

    let mut factor;
    let mut s;
    let mut diff;
    for _i in 0..3 {
        factor = rand::gen_range(0.3, 1.0);
        s = rand::gen_range(0.1, 1.);
        diff = if rand::gen_range(0, 100) < 50 {
            vec2(-(rotation.cos() * h / 4.0), -(rotation.sin() * h / 4.0))
        } else {
            vec2(rotation.cos() * h / 4.0, rotation.sin() * h / 4.0)
        };
        let pos = vec2(
            gs.player.pos.x - (h / 2.0 + (h / 3.0) * factor) * rotation.sin(),
            gs.player.pos.y + (h / 2.0 + (h / 3.0) * factor) * rotation.cos(),
        );
        gs.exhaust.push(Exhaust {
            created_at: time,
            pos: pos.add(diff * factor),
            size: 0.5 * s,
            vel: vec2(-(vel * rotation.sin()), vel * rotation.cos()).add(diff),
        });
    }
    gs.player.last_exhaust_frame = time;
}

pub fn stars(amount: i32, map_width: f32, map_height: f32) -> Vec<Star> {
    srand(421337421337);
    let mut stars = Vec::new();
    for _i in 0..amount {
        let sr = rand::gen_range(1, 10);
        let size;
        match sr {
            1 => size = 3.0,
            2 | 3 | 4 => size = 2.0,
            _ => size = 1.0,
        }
        stars.push(Star {
            pos: vec2(
                rand::gen_range(0.0, map_width),
                rand::gen_range(0.0, map_height),
            ),
            size,
        });
    }
    rand::srand(macroquad::miniquad::date::now() as _);

    stars
}
