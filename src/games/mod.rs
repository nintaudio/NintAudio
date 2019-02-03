use std::io::Cursor;

use clap::{clap_app, crate_authors, crate_description, crate_version};
use rodio::Decoder;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Assets;

mod breakout;
mod demo; // ça
mod mole_game;
mod pong;

pub fn audio(file: &str) -> Decoder<Cursor<std::borrow::Cow<'_, [u8]>>> {
    match Assets::get(file) {
        Some(content) => Decoder::new(Cursor::new(content)).unwrap(),
        None => panic!("File {:?} does not exist", file),
    }
}

pub fn once(device: &rodio::Device, file: &'static str, x: f32, y: f32) {
    let sink = rodio::Sink::new(device);
    sink.append(rodio::source::Spatial::new(
        audio(file),
        [x, y, 0.],
        [1., 0., 0.],  // left ear
        [-1., 0., 0.], // right ear
    ));
    sink.detach();
}

pub trait Game {
    fn update(&mut self, act: Option<Action>, device: &rodio::Device) -> Option<u32>;
}

#[derive(Debug)]
pub enum Action {
    Left,
    Right,
    Up,
    Fire,
    Quit,
}

pub fn select(device: &rodio::Device) -> Box<dyn Game> {
    let subcommands = vec![
        clap_app!(demo => (long_about: demo::description())(about: demo::about())), // ça
        clap_app!(breakout => (long_about: breakout::description())(about: breakout::about())),
        clap_app!(pong => (long_about: pong::description())(about: pong::about())),
        clap_app!(mole => (long_about: mole_game::description())(about: mole_game::about())),
    ];
    let clap = clap_app!(nintaudio =>
      (version: crate_version!())
      (author: crate_authors!())
      (about: crate_description!())
      (subcommands: subcommands)
    );

    match clap.get_matches().subcommand_name() {
        Some("demo") => Box::new(demo::new(device)), // ça
        Some("pong") => Box::new(pong::new(device)),
        Some("mole") => Box::new(mole_game::new(device)),
        Some("breakout") => Box::new(breakout::new(device)),
        None => {
            println!("Please provide a required game name as first argument");
            std::process::exit(1)
        }
        _ => unreachable!(),
    }
}
