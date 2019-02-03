use std::io::{stdin, stdout, Write};
use std::thread;
use std::time::Duration;

use rust_embed::RustEmbed;

mod games;
mod vessel;

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Assets;

fn main() {
    let device = rodio::default_output_device().unwrap();
    let mut game = games::select(&device);

    let rx = vessel::term::init();

    loop {
        let act = rx.try_recv();

        vessel::term::clear();

        if let Ok(games::Action::Quit) = act {
            println!("Good bye!");
            break;
        }

        if let Some(score) = game.update(act.ok(), &device) {

            vessel::term::cursor();

            println!("You made {} point(s)", score);
            break;
        }

        stdout.flush().unwrap();
        thread::sleep(Duration::from_millis(20));
    }
}
