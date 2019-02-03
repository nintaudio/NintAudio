const INITIAL_SIZE: (u32, u32) = (200, 200);

use std::rc::Rc;

use ai_behavior::{Action, Sequence};
use image::png;
use image::{ImageBuffer, ImageDecoder};
use piston_window::{OpenGL, PistonWindow, PressEvent, ResizeEvent, Size, Texture, TextureSettings, WindowSettings, Window, Input};
use sprite::*;
use std::sync::mpsc;
use std::thread;
use super::super::games;

use super::super::Assets;

pub fn init () -> std::sync::mpsc::Receiver<games::Action> {
    let (width, height) = INITIAL_SIZE;
    let mut window: PistonWindow = WindowSettings::new("piston: sprite", (width, height))
        .exit_on_esc(true)
        .opengl(OpenGL::V3_2)
        .build()
        .unwrap();

    let mut scene = Scene::new();
    let codec = png::PNGDecoder::new(&*Assets::get("logo.png").unwrap()).unwrap();
    let (w, h) = codec.dimensions();
    let buf = codec.read_image().unwrap();
    let img = ImageBuffer::from_raw(w as u32, h as u32, buf).unwrap();

    let tex =
        Rc::new(Texture::from_image(&mut window.factory, &img, &TextureSettings::new()).unwrap());
    let mut sprite = Sprite::from_texture(tex.clone());
    let Size { mut width, mut height } = window.size();
    sprite.set_position(f64::from(width) / 2., -f64::from(h as u32));
    sprite.set_scale(0.5, 0.5);

    let id = scene.add_child(sprite);

    // Run a sequence of animations.
    let seq = Sequence(vec![
        Action(Ease(
            EaseFunction::BounceOut,
            Box::new(MoveTo(1., f64::from(width) / 2., f64::from(height) / 2.)),
        )),
        Action(Ease(
            EaseFunction::CubicOut,
            Box::new(ScaleTo(1., 1., 1.)),
        )),
    ]);
    scene.run(id, &seq);

    // This animation and the one above can run in parallel.
    let rotate = Action(Ease(
        EaseFunction::ExponentialInOut,
        Box::new(RotateTo(2.0, 360.0)),
    ));
    scene.run(id, &rotate);

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
            while let Some(e) = window.next() {
        scene.event(&e);

        window.draw_2d(&e, |c, g| {
            piston_window::clear([1.0, 1.0, 1.0, 1.0], g);
            scene.draw(c.transform, g);
        });

        if let Some(key) = e.press_args() {
            println!("{:?}", key);
        }

        if let Some([w, h]) = e.resize_args() {
            width = w;
            height = h;
            // This animation and the one above can run in parallel.
            let recenter = Action(Ease(
                EaseFunction::ExponentialInOut,
                Box::new(MoveTo(1., f64::from(width) / 2., f64::from(height) / 2.)),
            ));
            scene.run(id, &recenter);
        }

        let i: Option<Input> = e.into();

        if let Some(event) = i {
            println!("{:?}", event);
        }
    }
});
rx
}
