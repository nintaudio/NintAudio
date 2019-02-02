use rodio::source::Source;

use super::{Action, Assets, Game};

const SPEED: i16 = 5;
const DEPTH: i16 = 800;

#[derive(Debug)]
struct Point {
    x: i16,
    y: i16,
}

// whatever you want
pub struct State {
    speed: Point,
    ball: Point,
    position: i8,
    sink: rodio::SpatialSink,
}

impl Game for State {
    fn update(&mut self, act: Option<Action>, device: &rodio::Device) -> Option<u32> {
        match act {
            Some(Action::Left) => self.position -= 1,
            Some(Action::Right) => self.position += 1,
            _ => {}
        };
        if self.position > 120 {
            self.position = 120;
            once!(device, "swing_miss_hit.mp3");
        }
        if self.position < -120 {
            self.position = -120;
            once!(device, "swing_miss_hit.mp3");
        }
        println!("{:?} b: {:?} p: {:?}", act, self.ball, self.position);
        self.sink
            .set_emitter_position([self.position as f32 / 10., 0., 0.]);
        None
    }
}

// Create a new game
pub fn new(device: &rodio::Device) -> State {
    let sink = rodio::SpatialSink::new(
        device,
        [0., 0., 0.],  // object
        [1., 0., 0.],  // left ear
        [-1., 0., 0.], // right ear
    );
    let source = audio!("music.ogg");
    sink.append(source.repeat_infinite());

    State {
        ball: Point { x: 0, y: DEPTH },
        speed: Point { x: 0, y: SPEED },
        position: 0,
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
