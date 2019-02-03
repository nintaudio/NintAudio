use rodio::source::Source;
use rand::Rng;

use super::{Action, Game, audio};

// whatever you want
pub struct Moles {
  left_count: u8,
  right_count: u8,
  position: i16,
  sink: rodio::SpatialSink,
  score: i16,
  spawn_time: u16,
  spawn_rate: u16,
  game_time: u16,
  moles: [bool; 3],
  last_mole: u8,

}

impl Game for Moles {
  fn update(&mut self, act: Option<Action>, _device: &rodio::Device) -> Option<u32> {
    self.game_time -= 1;

    self.spawn_time = if self.spawn_time == 0{
        //Remove last mole
        self.moles[self.last_mole as usize] = false;
        //unspawn sound

        loop{
            let slot = rand::thread_rng().gen_range(0,3) as usize;
            if !self.moles[slot] && (slot as u8) != self.last_mole{
                self.moles[slot] = true;
                self.last_mole = slot as u8;
                //spawn sound
                break;
            }
        }
        self.spawn_rate -= if self.spawn_rate > 25{
                1
            }else{
                0
            };
        //Return New Spawn Time
        self.spawn_rate
        }else{
        //Decrease Time
        self.spawn_time - 1
        };

    match act {
      Some(Action::Left) => {
            self.score += action_check(self.last_mole, 0);
        },
      Some(Action::Right) => {
            self.score += action_check(self.last_mole, 2);
        },
      Some(Action::Up) => {
            self.score += action_check(self.last_mole, 1);
        },
      _ => {},
    };

    if self.score < 0{
        self.score = 0;
    }

    println!("{:?} l: {} r: {} Score: {} Time: {} SpawnTime: {} Moles: {}, {}, {} Last: {}", 
             act, 
             self.left_count, 
             self.right_count, 
             self.score, 
             self.game_time/100, 
             self.spawn_time,
             if self.moles[0]{"1"}else{"0"},
             if self.moles[1]{"1"}else{"0"},
             if self.moles[2]{"1"}else{"0"},
             self.last_mole);
    
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

    let swing_hit = audio("swing_hit.mp3");
    let source = audio("object_movement.mp3");
    sink.append(source.repeat_infinite());

    Moles { 
        left_count: 0, 
        right_count: 0, 
        position: 0, 
        sink, 
        score: 0, 
        moles: [false; 3],
        last_mole: 1,
        spawn_time: 2_00, 
        spawn_rate: 1_00, 
        game_time: 60_00}

}

fn action_check(last_mole: u8, slot: u8) -> i16{
    if last_mole == slot{
        2
    }else{
        -1
    }
}

// One-line description
pub fn about() -> &'static str {
    "Hit as many moles as possible!"
}

// Complete description
pub fn description() -> &'static str {
    "Listen carefully and hit the moles when they pop out! Gain points by hitting them before they hide again, lose points by hitting an empty spot."
}
