use piston_window::*;
use piston_window::ButtonState;
use piston_window::Button;

pub fn start(mut state: crate::models::App) {
    let start_res = [500, 650];
    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", start_res)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let prop = start_res[0] as f64 / crate::FOV_SIZE as f64;

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
                let (cam_x, cam_y) = state.get_cam_offset();
                let (star_x, star_y) = (s.0 as f64, s.1 as f64);

                let plot_x = (star_x - cam_x) * prop as f64;
                let plot_y = (star_y - cam_y) * prop as f64;
                rectangle(
                    [1.0, 1.0, 1.0, 1.0], // red
                    [plot_x , plot_y, 1.0, 1.0],
                    context.transform,
                    graphics,
                );

            }
        });
    }
}
