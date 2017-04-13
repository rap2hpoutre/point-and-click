extern crate piston_window;

use piston_window::*;
use std::cmp;

const WIDTH: f64 = 320.0;
const HEIGHT: f64 = 200.0;

struct Player {
    x: f64,
    y: f64,
    objective_x: f64,
    objective_y: f64,
    speed: f64,
    is_moving: bool
}

impl Player {
    fn new() -> Player {
        Player {
            x:  WIDTH / 2.0,
            y:  HEIGHT / 2.0,
            objective_x:  WIDTH / 2.0,
            objective_y: HEIGHT / 2.0,
            is_moving: false,
            speed: 0.3,
        }
    }
    fn init_path_to_objective(&mut self, x: f64, y: f64) {
        self.is_moving = true;
        self.objective_x = x;
        self.objective_y = y;
    }
    fn move_to_objective(&mut self) {
        if self.objective_x as i32 == self.x as i32 && self.objective_y as i32 == self.y as i32 {
            self.is_moving = false;
        } else {
            if (self.objective_x as i32) < (self.x as i32) {
                self.x = self.x - self.speed;
            } else if (self.objective_x as i32) > (self.x as i32) {
                self.x = self.x + self.speed;
            }
            if (self.objective_y as i32) < (self.y as i32) {
                self.y = self.y - self.speed;
            } else if (self.objective_y as i32) > (self.y as i32) {
                self.y = self.y + self.speed;
            }
        }
    }
}

struct Mouse {
    x: f64,
    y: f64
}

impl Mouse {
    fn set_position(&mut self, x: f64, y: f64) {
        self.x = cmp::max(cmp::min(self.x as i32 + x as i32 - WIDTH as i32 / 2, WIDTH as i32 - 1), 0) as f64;
        self.y = cmp::max(cmp::min(self.y as i32 + y as i32 - HEIGHT as i32 / 2, HEIGHT as i32 - 1), 0) as f64;
    }
}

fn main() {

    let black = [0.0, 0.0, 0.0, 1.0];
    let grey = [0.5, 0.5, 0.5, 1.0];
    let white = [1.0, 1.0, 1.0, 1.0];

    let mut mouse = Mouse {
        x:  WIDTH / 2.0,
        y:  HEIGHT / 2.0,
    };

    let mut player = Player::new();

    let mut window: PistonWindow =
        WindowSettings::new("Hello star", [WIDTH as u32, HEIGHT as u32])
        .exit_on_esc(true).build().unwrap();

    window.set_capture_cursor(true);

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            
            clear([1.0; 4], g);

            rectangle(grey, [0.0, 0.0, WIDTH, HEIGHT], c.transform, g);

            rectangle(black, [player.x - 10.0, player.y - 20.0, 20.0, 20.0], c.transform, g);

            rectangle(white, [mouse.x, mouse.y -9.0, 2.0, 20.0], c.transform, g);
            rectangle(white, [mouse.x - 9.0, mouse.y, 20.0, 2.0], c.transform, g);

        });

        if let Some(button) = e.press_args() {
            player.init_path_to_objective(mouse.x, mouse.y);
            println!("PUSHED mouse button Button::Mouse({:?})", button);
        }

        if let Some(pos) = e.mouse_relative_args() {
            mouse.set_position(pos[0], pos[1]);
        }

        if player.is_moving {
            player.move_to_objective();
        }

    }
}