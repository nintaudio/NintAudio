use rodio::source::Source;

use super::{audio, once, Action, Game};

// whatever you want
pub struct Breakout {
    left_count: u8,
    right_count: u8,
    position: u8,
    sink: rodio::SpatialSink,
    bricks: [[bool; 3]; 6],
    ball_x: u8,
    ball_y: u8,
    hit_r_wall: bool,
    hit_top: bool,
    time: u32,
    points: u16,
}

impl Game for Breakout {
    fn update(&mut self, act: Option<Action>, device: &rodio::Device) -> Option<u32> {
        self.time += 1;

// Register input for movement of the bouncing tab
        match act {
            Some(Action::Left) => {
                self.left_count += 1;
                self.position -= 1;
            }
            Some(Action::Right) => {
                self.right_count += 1;
                self.position += 1;
            }
            _ => {}
        };

// Actions to be taken at every 50 tiks (move the ball, destroy bricks, emmit ponctual sound, et
        if self.time % 50 == 0 {

// Establish boolean parameter to true if the ball hit the right wall in order to change the
// direction of said ball
            if self.ball_x == 5 {
                self.hit_r_wall = true;

// Emmit sound when wall is hit
                once(
                    device,
                    "hit_wall.ogg",
                    (f32::from(self.ball_x) - f32::from(self.position)) / 2.,
                    f32::from(self.ball_y) / 2.,
                );

// Emmit sound and set boolean parameter if left wall is hit
            } else if self.ball_x == 0 {
                self.hit_r_wall = false;
                once(
                    device,
                    "hit_wall.ogg",
                    (f32::from(self.ball_x) - f32::from(self.position)) / 2.,
                    f32::from(self.ball_y) / 2.,
                );
            }

// When the ball is in the upper region of the game plane where the bricks are located,
            if self.ball_y > 4 && self.ball_y < 7 {
                println!("{} {}", self.ball_y, self.ball_x);
                if self.bricks[usize::from(self.ball_x)][usize::from(self.ball_y - 5)] == true {
                    self.points += 1;
                    self.bricks[usize::from(self.ball_x)][usize::from(self.ball_y - 5)] = false;
                    self.hit_top = true;
                    once(
                        device,
                        "short_bit_medium.ogg",
                        (f32::from(self.ball_x) - f32::from(self.position)) / 2.,
                        f32::from(self.ball_y) / 2.,
                    );
                }
            }

// Determining wether or not he ball hit the top and emmiting sound id top hit.

            if self.ball_y == 6 {
                self.hit_top = true;
                once(
                    device,
                    "hit_wall.ogg",
                    (f32::from(self.ball_x) - f32::from(self.position)) / 2.,
                    f32::from(self.ball_y) / 2.,
                );

// When the ball reach the bottom, determine if the player bouncing tab is
// at the ball's location (continue) and stop game if the player missed.
            } else if self.ball_y == 0 && self.position == self.ball_x {
                self.hit_top = false;
                once(
                    device,
                    "hit_paddle.ogg",
                    (f32::from(self.ball_x) - f32::from(self.position)) / 2.,
                    f32::from(self.ball_y) / 2.,
                );
            } else if self.ball_y == 0 {
                println!("You lost.");
                once(
                    device,
                    "Sounds_NintAudio/entity_passing_by.mp3",
                    (f32::from(self.ball_x) - f32::from(self.position)) / 2.,
                    f32::from(self.ball_y) / 2.,
                );
                return Some(self.points.into());
            }

// Time based trigger to determine if the player destroyed all of the available
// bricks (upper 2 rows)

            if self.time == 2800 {
                println!("You won!");
                return Some(self.points.into());
            }

// Execute movement of the ball in x/y coordinates depending on boolean hit
// right wall or top
            if !self.hit_r_wall {
                self.ball_x += 1;
            } else {
                self.ball_x -= 1;
            }
            if !self.hit_top {
                self.ball_y += 1;
            } else {
                self.ball_y -= 1;
            }

// Sets the position of the sound source according to the x and y coordinates
// of the ball relatively to the oberver
            self.sink.set_emitter_position([
                (f32::from(self.ball_x) - f32::from(self.position)) / 2.,
                ((f32::from(self.ball_y)) / 2.),
                0.,
            ]);
        }

        None
    }
}

// Create a new game
pub fn new(device: &rodio::Device) -> Breakout {
    let sink = rodio::SpatialSink::new(
        device,
        [0., 0., 0.],  // object
        [1., 0., 0.],  // left ear
        [-1., 0., 0.], // right ear
    );
    let source = audio("object_movement_big.ogg");

    sink.append(source.repeat_infinite());

    Breakout {
        left_count: 0,
        right_count: 0,
        position: 2,
        sink,
        bricks: [[true; 3]; 6],
        ball_x: 2,
        ball_y: 0,
        hit_top: false,
        hit_r_wall: false,
        time: 0,
        points: 0,
    }
}

// One-line description
pub fn about() -> &'static str {
    "A"
}

// Complete description
pub fn description() -> &'static str {
    "A long"
}
