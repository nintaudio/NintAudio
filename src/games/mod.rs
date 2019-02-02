use clap::{clap_app, crate_authors, crate_description, crate_version};

mod demo;
mod breakout;

pub trait Game {
    fn update(&mut self, act: Option<Action>, device: &rodio::Device) -> bool;
}

#[derive(Debug)]
pub enum Action {
    Left,
    Right,
    Fire,
    Quit,
}

pub fn select(device: &rodio::Device) -> Box<dyn Game> {
    let subcommands = vec![
        clap_app!(demo => (long_about: demo::description())(about: demo::about())),
        clap_app!(breakout => (long_about: breakout::description())(about: breakout::about())),
    ];
    let mat = clap_app!(nintaudio =>
      (version: crate_version!())
      (author: crate_authors!())
      (about: crate_description!())
      (subcommands: subcommands)
    )
    .get_matches();

    match mat.subcommand_name() {
        Some("demo") => Box::new(demo::new(device)),
        Some("breakout") => Box::new(breakout::new(device)),
        None => panic!("Please provide a required subcommand"),
        _ => unreachable!(),
    }
}
