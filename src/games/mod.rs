use clap::{clap_app, crate_authors, crate_description, crate_version};

mod demo;

pub trait Game {
    fn update(&mut self, act: Option<Action>) -> bool;
}

#[derive(Debug)]
pub enum Action {
    Left,
    Right,
    Fire,
    Quit,
}

pub fn select() -> Box<dyn Game> {
    let subcommands = vec![
        clap_app!(demo => (long_about: demo::description())(about: demo::about())),
    ];
    let mat = clap_app!(nintaudio =>
      (version: crate_version!())
      (author: crate_authors!())
      (about: crate_description!())
      (subcommands: subcommands)
    )
    .get_matches();

    match mat.subcommand_name() {
        Some("demo") => Box::new(demo::new()),
        None => panic!("Please provide a required subcommand"),
        _ => unreachable!(),
    }
}
