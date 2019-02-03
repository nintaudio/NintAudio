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
        if cfg!(piston) {
            vessel::piston::refresh();
        } else {
            vessel::term::refresh();
        }
        let act = rx.try_recv().ok();

        if let Some(games::Action::Quit) = act {
            if cfg!(piston) {
                vessel::piston::clear();
            } else {
                vessel::term::clear();
            }
            println!("Good bye!");
            std::process::exit(0);
        }

        if let Some(score) = game.update(act, &device) {
            if cfg!(piston) {
                vessel::piston::clear();
            } else {
                vessel::term::clear();
            }
            println!("You made {} point(s)", score);
            std::process::exit(0);
        }

        thread::sleep(Duration::from_millis(20));
    });
    
    if cfg!(piston) {
        vessel::piston::init(tx);
    } else {
        vessel::term::init(tx);
    }
}
