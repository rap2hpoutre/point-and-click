extern crate piston_window;

use piston_window::*;

const WIDTH: u32 = 320;
const HEIGHT: u32 = 200;

fn main() {

    let black = [0.0, 0.0, 0.0, 1.0];
    let white = [1.0, 1.0, 1.0, 1.0];
    let mut x: f64 = 160.0;
    let mut y: f64 = 100.0;

    let mut window: PistonWindow =
        WindowSettings::new("Hello star", [WIDTH, HEIGHT])
        .exit_on_esc(true).build().unwrap();

    window.set_capture_cursor(true);

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([1.0; 4], g);
            rectangle(black, [0.0, 0.0, WIDTH as f64, HEIGHT as f64], c.transform, g);
            rectangle(white, [x, y -9.0, 2.0, 20.0], c.transform, g);
            rectangle(white, [x - 9.0, y, 20.0, 2.0], c.transform, g);
        });

        if let Some(button) = e.press_args() {
            if button == Button::Mouse(MouseButton::Left) {
                println!("PUSHED mouse button '{:?}'", button);
            }
        };

//        if let Some(pos) = e.mouse_cursor_args() {
//            x = pos[0] as f64;
//            y = pos[1] as f64;
//        }


        if let Some(pos) = e.mouse_relative_args() {
            x = x + pos[0] as f64 - 160.0;
            y = y + pos[1] as f64 - 100.0;
            if x >= 320.0 {
                x = 320.0;
            }
            if x < 0.0 {
                x = 0.0;
            }
            if y >= 200.0 {
                y = 200.0;
            }
            if y < 0.0 {
                y = 0.0;
            }
        }

    }
}