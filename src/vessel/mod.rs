use super::games;

pub mod term;
pub mod piston;

pub trait Vessel {
  fn refresh(&mut self) -> Option<games::Action>;
  fn clear(&self);
}
