extern crate piston_window;
use piston_window::*;

mod enums;

fn main() {
    enums::main();

    // Play with piston
    let mut window: PistonWindow = WindowSettings::new("Hey Window", [640, 640])
        .exit_on_esc(true)
        .build()
        .unwrap();
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);
            rectangle(
                [1.0, 0.0, 0.0, 1.0],
                [0.0, 0.0, 100.0, 100.0],
                context.transform,
                graphics,
            )
        });
    }
}
