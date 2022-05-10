const ASTEROID_MAX_SIZE: i8 = 3;

struct Point {
    x: f32,
    y: f32,
}

struct Asteroid {
    pos: Point,
    vel: Point,
    size: i8,
}

struct Spaceship {
    w: f32,
    h: f32,
    pos: Point,
    rot: f32,
    vel: Point,
    acc: f32,
}
struct GameState {
    scale: f32,
    player: Spaceship,
    asteroids: Vec<Asteroid>,
    lives: i8,
}

#[macroquad::main("Pong")]
async fn main() {
    //init stuff
    loop {
        println!("Hello, world!");
    }
}
