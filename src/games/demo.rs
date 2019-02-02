use super::{Game, Action};

#[derive(Debug)]
pub struct Demo {
  left_count: u8,
  right_count: u8,
}

impl Game for Demo {
  fn update(&mut self, act: Option<Action>) -> bool {
    match act {
      Some(Action::Left) => self.left_count += 1,
      Some(Action::Right) => self.right_count += 1,
      _ => {},
    };
    println!("{:?} l: {} r: {}", act, self.left_count, self.right_count);
    false
  }
}

pub fn new() -> Demo {
    Demo { left_count: 0, right_count: 0 }
}

pub fn about() -> &'static str {
    "A"
}

pub fn description() -> &'static str {
    "A long"
}
