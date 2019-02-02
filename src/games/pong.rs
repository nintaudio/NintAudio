use std::io::Cursor;

use rodio::source::Source;
use rust_embed::RustEmbed;

use super::{Game, Action};

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Assets;

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
  fn update(&mut self, act: Option<Action>, device: &rodio::Device) -> bool {
    match act {
      Some(Action::Left) => self.position -= 1,
      Some(Action::Right) => self.position += 1,
      _ => {},
    };
    if self.position > 120 {
      self.position = 120;
      rodio::play_once(device, Cursor::new(Assets::get("swing_miss_hit.mp3").unwrap())).unwrap().detach();
    }
    if self.position < -120 {
      self.position = -120;
      rodio::play_once(device, Cursor::new(Assets::get("swing_miss_hit.mp3").unwrap())).unwrap().detach();
    }
    println!("{:?} b: {:?} p: {:?}", act, self.ball, self.position);
    self.sink.set_emitter_position([self.position as f32 / 10., 0., 0.]);
    false
  }
}

// Create a new game
pub fn new(device: &rodio::Device) -> State {
    let sink = rodio::SpatialSink::new(
        device,
        [ 0., 0., 0.], // object
        [ 1., 0., 0.], // left ear
        [-1., 0., 0.], // right ear
    );
    let source = rodio::Decoder::new(Cursor::new(Assets::get("music.ogg").unwrap())).unwrap();
    sink.append(source.repeat_infinite());

    State { ball: Point{ x: 0, y: DEPTH }, speed: Point{ x: 0, y: SPEED }, position: 0, sink }
}

// One-line description
pub fn about() -> &'static str {
    "Move to intercept the ball"
}

// Complete description
pub fn description() -> &'static str {
    "Move left or right to intercept the ball and send it back to the wall. The longer you stand, the more point you get"
}
