use super::{Asteroid, ASTEROID_VEL, PLAYER_WIDTH};
use macroquad::prelude::{rand, vec2, Vec2};

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
