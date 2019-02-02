use std::io::Cursor;

use rodio::source::Source;
use rust_embed::RustEmbed;

use super::{Game, Action};

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Assets;

// whatever you want
pub struct Demo {
  left_count: u8,
  right_count: u8,
  position: i16,
  sink: rodio::SpatialSink,
}

impl Game for Demo {
  fn update(&mut self, act: Option<Action>, _device: &rodio::Device) -> bool {
    match act {
      Some(Action::Left) => {
            self.left_count += 1;
            self.position -= 1;
        },
      Some(Action::Right) => {
            self.right_count += 1;
            self.position += 1;
        },
      _ => {},
    };
    println!("{:?} l: {} r: {}", act, self.left_count, self.right_count);
    self.sink.set_emitter_position([self.position as f32 / 10., 0., 0.]);
    false
  }
}

// Create a new game
pub fn new(device: &rodio::Device) -> Demo {
    let sink = rodio::SpatialSink::new(
        device,
        [ 0., 0., 0.], // object
        [ 1., 0., 0.], // left ear
        [-1., 0., 0.], // right ear
    );
    let source = rodio::Decoder::new(Cursor::new(Assets::get("music.ogg").unwrap())).unwrap();
    sink.append(source.repeat_infinite());

    Demo { left_count: 0, right_count: 0, position: 0, sink }
}

// One-line description
pub fn about() -> &'static str {
    "A"
}

// Complete description
pub fn description() -> &'static str {
    "A long"
}
