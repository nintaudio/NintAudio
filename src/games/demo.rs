use rodio::source::Source;

use super::{Action, Assets, Game};

// whatever you want
pub struct State {
    left_count: u8,
    right_count: u8,
    position: i16,
    sink: rodio::SpatialSink,
}

impl Game for State {
    fn update(&mut self, act: Option<Action>, _device: &rodio::Device) -> Option<u32> {
        match act {
            Some(Action::Left) => {
                self.left_count += 1;
                self.position -= 1;
            }
            Some(Action::Right) => {
                self.right_count += 1;
                self.position += 1;
            }
            _ => {}
        };
        println!("{:?} l: {} r: {}", act, self.left_count, self.right_count);
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
        left_count: 0,
        right_count: 0,
        position: 0,
        sink,
    }
}

// One-line description
pub fn about() -> &'static str {
    "A"
}

// Complete description
pub fn description() -> &'static str {
    "A long"
}
