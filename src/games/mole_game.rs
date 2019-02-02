use std::io::Cursor;

use rodio::source::Source;
use rust_embed::RustEmbed;

use super::{Game, Action};

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Assets;

// whatever you want
pub struct Moles {
  left_count: u8,
  right_count: u8,
  position: i16,
  sink: rodio::SpatialSink,
  point: u8,
}

impl Game for Moles {
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
      Some(Action::Fire) => {
            self.point += 1;
        },
      _ => {},
    };

    println!("{:?} l: {} r: {}\tPoints: {}", act, self.left_count, self.right_count, self.point);
    self.sink.set_emitter_position([self.position as f32 / 10., 0., 0.]);
    false
  }
}

// Create a new game
pub fn new(device: &rodio::Device) -> Moles {
    let sink = rodio::SpatialSink::new(
        device,
        [ 0., 0., 0.], // object
        [ 1., 0., 0.], // left ear
        [-1., 0., 0.], // right ear
    );
    let source = rodio::Decoder::new(Cursor::new(Assets::get("enemy_spawn.mp3").unwrap())).unwrap();
    sink.append(source.repeat_infinite());

    Moles { left_count: 0, right_count: 0, position: 0, sink, point: 0}
    
}

// One-line description
pub fn about() -> &'static str {
    "Hit as many moles as possible!"
}

// Complete description
pub fn description() -> &'static str {
    "Listen carefully and hit the moles when they pop out! Gain points by hitting them before they hide again, lose points by hitting an empty spot."
}
