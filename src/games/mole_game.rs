use rodio::source::Source;

use super::{Action, Assets, Game};

// whatever you want
pub struct Moles {
  left_count: u8,
  right_count: u8,
  position: i16,
  sink: rodio::SpatialSink,
  score: u8,
  spawn_time: u16,
  spawn_rate: u16,
  game_time: u16,

}

impl Game for Moles {
  fn update(&mut self, act: Option<Action>, _device: &rodio::Device) -> Option<u32> {
    self.game_time -= 1;
    self.spawn_time = if self.spawn_time == 0{
        self.spawn_rate
        }else{
        self.spawn_time - 1
        };

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
            self.score += 1;
        },
      _ => {},
    };

    println!("{:?} l: {} r: {} Score: {} Time: {} SpawnTime: {}", 
             act, 
             self.left_count, 
             self.right_count, 
             self.score, 
             self.game_time/100, 
             self.spawn_time);
    
    self.sink.set_emitter_position([self.position as f32 / 10., 0., 0.]);
    None
  }
}

// Create a new game
pub fn new(device: &rodio::Device) -> Moles {
    let sink = rodio::SpatialSink::new(
        device,
        [0., 0., 0.],  // object
        [1., 0., 0.],  // left ear
        [-1., 0., 0.], // right ear
    );
    let source = audio!("enemy_spawn.mp3");
    sink.append(source.repeat_infinite());

    Moles { 
        left_count: 0, 
        right_count: 0, 
        position: 0, 
        sink, score: 0, 
        spawn_time: 5_00, 
        spawn_rate: 5_00, 
        game_time: 60_00}
    
}

// One-line description
pub fn about() -> &'static str {
    "Hit as many moles as possible!"
}

// Complete description
pub fn description() -> &'static str {
    "Listen carefully and hit the moles when they pop out! Gain points by hitting them before they hide again, lose points by hitting an empty spot."
}
