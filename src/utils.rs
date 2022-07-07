use macroquad::prelude::{screen_height, screen_width, Vec2};

pub fn wrap(pos: Vec2, width: f32, height: f32) -> Vec2 {
    let mut new_pos = pos;
    if pos.x > screen_width() {
        new_pos.x = 0.0 - width;
    } else if new_pos.x < 0.0 - width {
        new_pos.x = screen_width();
    }

    if new_pos.y > screen_height() {
        new_pos.y = 0.0 - height;
    } else if new_pos.y < 0.0 - height {
        new_pos.y = screen_height();
    }

    new_pos
}

/*
    Line segment intersection algorithm.
    The lines AB and CD intersect if and only if points A and B are separated
    by segment CD and points C and D are separated by segment AB. If points
    A and B are separated by segment CD then ACD and BCD should have opposite
    orientation meaning either ACD or BCD is counterclockwise but not both.
    ref: https://bryceboe.com/2006/10/23/line-segment-intersection-algorithm/
*/
pub fn ccw(a: Vec2, b: Vec2, c: Vec2) -> bool {
    (c.y - a.y) * (b.x - a.x) > (b.y - a.y) * (c.x - a.x)
}

pub fn intersects(a: Vec2, b: Vec2, c: Vec2, d: Vec2) -> bool {
    ccw(a, c, d) != ccw(b, c, d) && ccw(a, b, c) != ccw(a, b, d)
}
