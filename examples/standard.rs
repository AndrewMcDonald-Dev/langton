use langton_terminal::{Color, Position, World};

const PIXELS_PER_CELL: f64 = 4.0;
const WINDOW_WIDTH: u32 = 1000;
const WINDOW_HEIGHT: u32 = 1000;
const STEP_SIZE: i32 = 5 * PIXELS_PER_CELL as i32;

// extern crate piston_window;
use piston_window::*;
fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Langton's Ant", (WINDOW_WIDTH, WINDOW_HEIGHT))
            .exit_on_esc(true)
            .build()
            .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));
    println!("Use WASD or Arrow keys to move around the world!");

    let mut world = World::default();
    let mut left = -(WINDOW_WIDTH as i32 / PIXELS_PER_CELL as i32) / 2;
    let mut top = (WINDOW_HEIGHT as i32 / PIXELS_PER_CELL as i32) / 2;
    while let Some(e) = window.next() {
        if let Some(button) = e.press_args() {
            left += match button {
                Button::Keyboard(Key::Left | Key::A) => -STEP_SIZE,
                Button::Keyboard(Key::Right | Key::D) => STEP_SIZE,
                _ => 0,
            };

            top += match button {
                Button::Keyboard(Key::Down | Key::S) => -STEP_SIZE,
                Button::Keyboard(Key::Up | Key::W) => STEP_SIZE,
                _ => 0,
            };
        } else {
            let step = world.next();
            let Size { width, height } = window.size();
            window.draw_2d(&e, move |c, g, _| {
                if let Some(world) = step {
                    println!("{}", world.cells.len());
                    clear([1.0, 1.0, 1.0, 1.0], g);

                    for y in 0..(height / PIXELS_PER_CELL) as i32 {
                        for x in left..(left + (width / PIXELS_PER_CELL) as i32) {
                            let pos = Position(x, top - y);
                            let color = world.get_color(pos);

                            if color != Color::White {
                                let color = color.to_values();
                                rectangle(
                                    color,
                                    [
                                        (x - left) as f64 * PIXELS_PER_CELL,
                                        y as f64 * PIXELS_PER_CELL,
                                        PIXELS_PER_CELL,
                                        PIXELS_PER_CELL,
                                    ],
                                    c.transform,
                                    g,
                                );
                            }
                        }
                    }
                }
            });
        }
    }
}
