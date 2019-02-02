use clap::{clap_app, crate_authors, crate_description, crate_version};

mod demo; // ça
mod pong;

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
        clap_app!(demo => (long_about: demo::description())(about: demo::about())), // ça
        clap_app!(pong => (long_about: pong::description())(about: pong::about())),
    ];
    let clap = clap_app!(nintaudio =>
      (version: crate_version!())
      (author: crate_authors!())
      (about: crate_description!())
      (subcommands: subcommands)
    );

    match clap.get_matches().subcommand_name() {
        Some("demo") => Box::new(demo::new(device)), // ça
        Some("pong") => Box::new(pong::new(device)), // ça
        None => {
            println!("Please provide a required game name as first argument");
            std::process::exit(1)
        },
        _ => unreachable!(),
    }
}
