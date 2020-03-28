use piston_window::*;
use piston_window::Input;
use piston_window::ButtonState;
use piston_window::Button;
use piston_window::Key;

pub fn start(mut state: crate::models::App) {
    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();

    while let Some(event) = window.next() {
        
        if let Some(k) = event.button_args() {
            if k.state == ButtonState::Press {
                match k.button {
                    Button::Keyboard(d) => state.rotate(d),
                    _ => (),
                }
            }
        }

        window.draw_2d(&event, |context, graphics, _device| {
            clear([0.0; 4], graphics);

            for s in state.cam_map() {
                rectangle(
                    [1.0, 1.0, 1.0, 1.0], // red
                    [(s.0 as f64 - state.get_cam_offset().0) , (s.1 as f64 - state.get_cam_offset().1), 1.0, 1.0],
                    context.transform,
                    graphics,
                );

            }
        });
    }
}
