use piston_window::*;
use piston_window::ButtonState;
use piston_window::Button;

pub fn start(mut state: crate::models::App) {
    let start_res = [500, 650];
    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", start_res)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let font_size = 12;
    let mut char_cache = window.load_font("/usr/share/fonts/liberation/LiberationMono-Regular.ttf").unwrap();


    let prop = start_res[0] as f64 / crate::FOV_SIZE as f64;
    let ui_start_p = (crate::FOV_SIZE as f64 * prop) + font_size as f64 + 10.0;

    let mut mouse_pos = [0.0, 0.0];
    let mut estrela_sel: Option<crate::models::Ponto> = None;
    while let Some(event) = window.next() {
        
        
        if let Some(pos) = event.mouse_cursor_args() {
            mouse_pos = pos;
        }

        window.draw_2d(&event, |context, graphics, device| {
            clear([0.0; 4], graphics);
            let cam_map = state.cam_map();

            // Estrelas
            for s in &cam_map {
                let (cam_x, cam_y) = state.get_cam_offset();
                let (star_x, star_y) = (s.0 as f64, s.1 as f64);

                let plot_x = (star_x - cam_x) * prop as f64;
                let plot_y = (star_y - cam_y) * prop as f64;

                let tam = s.2.size();

                ellipse(
                    [1.0, 1.0, 1.0, s.2.mag()],
                    [plot_x , plot_y, tam, tam],
                    context.transform,
                    graphics,
                );

            }

            // UI
            let transform = context.transform.trans(10.0, ui_start_p);

            text::Text::new_color([1.0, 1.0, 1.0, 1.0], font_size).draw(
                &format!("Quantidade \nde estrelas na câmera: {}",
                          cam_map.len(),
                          ),
                &mut char_cache,
                &context.draw_state,
                transform, graphics
            ).unwrap();

            let transform = context.transform.trans(10.0, ui_start_p + font_size as f64 + 5.0);
            text::Text::new_color([1.0, 1.0, 1.0, 1.0], font_size).draw(
                &format!("Estrela selecionada: {:?}", 
                          estrela_sel
                          ),
                &mut char_cache,
                &context.draw_state,
                transform, graphics
            ).unwrap();
            char_cache.factory.encoder.flush(device);

        });

        if let Some(k) = event.button_args() {
            if k.state == ButtonState::Press {
                match k.button {
                    Button::Keyboard(d) => state.rotate(d),
                    Button::Mouse(_c) => {
                        let range_x = mouse_pos[0] + 50.0; 
                        let range_neg_x = mouse_pos[0] - 50.0; 
                        let range_y = mouse_pos[1] + 50.0; 
                        let range_neg_y = mouse_pos[1] - 50.0; 

                        dbg!(&range_neg_x);
                        dbg!(&range_x);
                        dbg!(&range_neg_y);
                        dbg!(&range_y);

                        let (cam_x, cam_y) = state.get_cam_offset();
                        for s in state.cam_map() {
                            let (star_x, star_y) = (s.0 as f64, s.1 as f64);
                            let plot_x = (star_x - cam_x) * prop as f64;
                            let plot_y = (star_y - cam_y) * prop as f64;

                            dbg!(plot_x);
                            dbg!(plot_y);

                            if plot_x >= range_neg_x &&
                                plot_x <= range_x &&
                                plot_y >= range_neg_y &&
                                plot_y <= range_y {
                                
                                dbg!(&s);
                                estrela_sel = Some(s.2.clone());
                            }
                        }
                    },
                    _ => (),
                }
            }  
        }

    }
}
