const INITIAL_SIZE: (u32, u32) = (200, 200);

use std::rc::Rc;

use ai_behavior::{Action, Sequence};
use image::png;
use image::{ImageBuffer, ImageDecoder};
use piston_window::{
    OpenGL, PistonWindow, PressEvent, ResizeEvent, Size, Texture, TextureSettings, Window, Key,
    WindowSettings,
};
use sprite::*;

use super::super::{games, Assets};

pub fn init(tx: &std::sync::mpsc::Sender<games::Action>) {
    let mut window: PistonWindow = WindowSettings::new("nintaudio", INITIAL_SIZE)
        .exit_on_esc(true)
        .opengl(OpenGL::V3_2)
        .build()
        .unwrap();

    let mut scene = Scene::new();
    let logo = Assets::get("logo.png").unwrap();
    let codec = png::PNGDecoder::new(&*logo).unwrap();
    let (w, h) = codec.dimensions();
    let buf = codec.read_image().unwrap();
    let img = ImageBuffer::from_raw(w as u32, h as u32, buf).unwrap();

    let tex =
        Rc::new(Texture::from_image(&mut window.factory, &img, &TextureSettings::new()).unwrap());
    let mut sprite = Sprite::from_texture(tex.clone());
    let Size { width, height } = window.size();
    sprite.set_position(width / 2., -f64::from(h as u32));
    sprite.set_scale(0.5, 0.5);

    let id = scene.add_child(sprite);

    // Run a sequence of animations.
    let seq = Sequence(vec![
        Action(Ease(
            EaseFunction::BounceOut,
            Box::new(MoveTo(1., width / 2., height / 2.)),
        )),
        Action(Ease(EaseFunction::CubicOut, Box::new(ScaleTo(1., 1., 1.)))),
    ]);
    scene.run(id, &seq);

    // This animation and the one above can run in parallel.
    let rotate = Action(Ease(
        EaseFunction::ExponentialInOut,
        Box::new(RotateTo(2., 360.0)),
    ));
    scene.run(id, &rotate);

    while let Some(e) = window.next() {
        scene.event(&e);

        window.draw_2d(&e, |c, g| {
            piston_window::clear([10.0, 10.0, 1.0, 1.0], g);
            scene.draw(c.transform, g);
        });

        if let Some([w, h]) = e.resize_args() {
            // This animation and the one above can run in parallel.
            let recenter = Action(Ease(
                EaseFunction::ExponentialInOut,
                Box::new(MoveTo(1., w / 2., h / 2.)),
            ));
            scene.run(id, &recenter);
        }

        match e.press_args()  {
            Some(piston_window::Button::Keyboard(k)) => {
                match k {
                    Key::Q => Some(games::Action::Quit),
                    Key::Left | Key::A | Key::D4 => Some(games::Action::Left),
                    Key::Right | Key::D | Key::D6 => Some(games::Action::Right),
                    Key::Return | Key::Space | Key::D5 => Some(games::Action::Fire),
                    Key::Up | Key::W | Key::D8 => Some(games::Action::Up),
                    _ => None,
                }
            },
            _ => None,
        }
        .and_then(|m| Some(tx.send(m).unwrap()));
    }
}

pub fn refresh() {}

// Show the cursor again before we exit.
pub fn clear() {}
