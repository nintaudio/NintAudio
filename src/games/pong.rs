use rodio::source::Source;
use std::ops::AddAssign;

use super::{audio, once, Action, Game};

const SPEED: i16 = 5;
const DEPTH: i16 = 800;
const WIDTH: i16 = 400;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i16,
    y: i16,
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Point) {
        *self = Point {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

// whatever you want
pub struct State {
    points: u32,
    speed: Point,
    ball: Point,
    position: i16,
    sink: rodio::SpatialSink,
}

impl Game for State {
    fn update(&mut self, act: Option<Action>, device: &rodio::Device) -> Option<u32> {
        match act {
            Some(Action::Left) => self.position -= 1,
            Some(Action::Right) => self.position += 1,
            _ => {}
        };

        if self.position > 40 {
            self.position = 40;
            once(device, "wall_hit.ogg", 0., 0.);
        }
        if self.position < -40 {
            self.position = -40;
            once(device, "wall_hit.ogg", 0., 0.);
        }

        self.ball += self.speed;

        if self.ball.x > WIDTH {
            self.ball.x = WIDTH;
            self.speed.x = -self.speed.x;
            once(
                device,
                "wall_hit.ogg",
                f32::from(self.ball.x - self.position),
                f32::from(self.ball.y),
            );
        }
        if self.ball.x < -WIDTH {
            self.ball.x = -WIDTH;
            self.speed.x = -self.speed.x;
            once(
                device,
                "wall_hit.ogg",
                f32::from(self.ball.x - self.position),
                f32::from(self.ball.y),
            );
        }

        if self.ball.y == 0 {
            if self.ball.x < self.position - 10 || self.ball.x > self.position + 10 {
                return Some(self.points);
            }
            self.points += 1;
            self.speed.y = SPEED;
        }

        if self.ball.y == DEPTH {
            self.speed.y = -SPEED;
            once(device, "wall_hit.ogg", 0., 0.);
        }

        println!("{:?} b: {:?} p: {:?}", act, self.ball, self.position);
        self.sink.set_emitter_position([
            f32::from(self.ball.x - self.position) / 100.,
            f32::from(self.ball.y) / 100.,
            0.,
        ]);
        None
    }
}

// Create a new game
pub fn new(device: &rodio::Device) -> State {
    let sink = rodio::SpatialSink::new(
        device,
        [0., 0., 0.],   // object
        [10., 0., 0.],  // left ear
        [-10., 0., 0.], // right ear
    );
    let source = rodio::source::SineWave::new(220);
    sink.append(source.repeat_infinite());

    State {
        ball: Point { x: 0, y: DEPTH },
        speed: Point {
            x: SPEED,
            y: -SPEED,
        },
        position: 0,
        points: 0,
        sink,
    }
}

// One-line description
pub fn about() -> &'static str {
    "Move to intercept the ball"
}

// Complete description
pub fn description() -> &'static str {
    "Move left or right to intercept the ball and send it back to the wall. The longer you stand, the more point you get"
}
