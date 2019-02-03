use std::sync::mpsc;
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

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || loop {
        vessel::piston::refresh();
        let act = rx.try_recv().ok();

        if let Some(games::Action::Quit) = act {
            vessel::piston::clear();
            println!("Good bye!");
            std::process::exit(0);
        }

        if let Some(score) = game.update(act, &device) {
            vessel::piston::clear();
            println!("You made {} point(s)", score);
            std::process::exit(0);
        }

        thread::sleep(Duration::from_millis(20));
    });

    vessel::piston::init(tx);
}
