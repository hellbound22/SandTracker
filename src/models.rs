use piston_window::Key;

use std::collections::HashMap;

use crate::models;
use crate::CANVAS_SIZE;

#[derive(Debug, Clone)]
pub struct Camera {
    pub r_asc: f64,
    pub dec: f64,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {r_asc: 20000.0, dec: 0.0}
    }
}

pub struct App {
    cam: Camera,
    canvas: Canvas,
}

impl App {
    pub fn get_cam_offset(&self) -> (f64, f64) {
        let x = self.cam.r_asc;
        let y = self.cam.dec;

        (x, y)
    }

    pub fn new(canvas: Canvas) -> App {
        App {
            cam: Camera::new(),
            canvas
        }
    }

    pub fn cam_map(&self) -> Vec<(i64, i64, Ponto)> {
        let range_v = crate::FOV_SIZE as f64 + self.cam.dec;
        let range_h = crate::FOV_SIZE as f64 + self.cam.r_asc;
        let mut on_screen: Vec<(i64, i64, Ponto)> = Vec::new();

        for (_k, s) in self.canvas.map.iter() {
            let x = (s.r_asc.to_degrees() * 60.0).round();
            // problemas de dec negativo
            // TODO: ISSO PROVAVELMENTE ESTÁ ERRADO
            let y = (s.dec.to_degrees() * 60.0).round();

            if x >= self.cam.r_asc && 
                    x <= range_h && 
                    y >= self.cam.dec && 
                    y <= range_v  {

                on_screen.push(((s.r_asc.to_degrees() * 60.0).round() as i64, (s.dec.to_degrees() * 60.0).round() as i64, s.clone()));
            }

        }
        
        on_screen
    }

    pub fn rotate(&mut self, dir: Key) {
        match dir {
            Key::Right => {
                if self.cam.r_asc < (360.0 * 60.0) - 200.0 {
                    self.cam.r_asc += crate::STEP as f64;    
                }
            },
            Key::Left => {
                let x = self.cam.r_asc - crate::STEP as f64;
                if x > -25.0 {
                    self.cam.r_asc = x;
                }
            },
            Key::Up => {
                if self.cam.dec > (-90.0 * 60.0) {
                    self.cam.dec -= crate::STEP as f64;    
                }
            },
            Key::Down => {
                if self.cam.dec < (90.0 * 60.0) {
                    self.cam.dec += crate::STEP as f64;    
                }
            },
            _ => {},
        }

    }
}

#[derive(Debug, Clone)]
pub struct Canvas {
    pub size: i64,
    pub map: HashMap<(i64, i64), models::Ponto>,
}

impl Canvas {
    pub fn _get_vec(&self, h: i64, v: i64) -> Vec<bool> {
        let mut r: Vec<bool> = Vec::new();

        if h == -1 {
            for x in 0..CANVAS_SIZE as i64 {
                r.push(match self.map.get(&(x, v)) {
                    Some(_) => true,
                    None => false,
                });
            }
        }

        if v == -1 {
            for x in 0..CANVAS_SIZE as i64 {
                r.push(match self.map.get(&(h, x)) {
                    Some(_) => true,
                    None => false,
                });
            }
        }
        r
    }
}

#[derive(Debug, Clone)]
pub struct Ponto {
    id: i32,        // Identificador Hipparcos
    pub r_asc: f64, // Ascensão Reta
    pub dec: f64,   // Declinação
    plx: f64,       // Paralaxe
    pub pos_h: i64, // Posição na horizontal normalizada (0-21600)
    pub pos_v: i64, // Posição na vertical normalizada (0-21600)
    pub mag: f64,   // Magnetude (Brilho)
}

impl Ponto {
    pub fn size(&self) -> f64 {
        let x = (crate::DEFAULT_STAR_SIZE * self.mag()) as f64;
        if x < 1.0 {
            1.0
        } else { 
            x
        }
    }
    pub fn mag(&self) -> f32 {
        let cn = (1.0/(16.0 + 4.0)) as f32 ; // Escala
        let f = |x: f32| {
            ((x * cn) * -1.0) + 1.0
        };

        let pre = f(self.mag as f32);
        if pre <= 0.05 {
            return 0.05
        } else if pre >= 1.0 {
            return 1.0
        } else {
            return pre
        }
    }

    pub fn coord(&self) -> (i64, i64) {
        (self.pos_h, self.pos_v)
    }

    pub fn new_from_line(l: &str) -> Result<Ponto, std::num::ParseFloatError> {
        let id: i32 = l
            .get(0..6)
            .unwrap()
            .trim()
            .parse()
            .expect("Erro de parsing");
        let r_asc: f64 = l.get(15..28).unwrap().trim().parse()?;
        let dec: f64 = l.get(29..42).unwrap().trim().parse()?;
        let plx: f64 = l.get(43..50).unwrap().trim().parse()?;
        let mag: f64 = l.get(129..136).unwrap().trim().parse()?;

        // Determina a posição no mapa
        // Utiliza Projeção de Miller
        let pos_h = (r_asc.to_degrees() * 60.0).round() as i64;

        let mut pos_v = dec.to_degrees().abs() * 60.0;
        if dec < 0.0 {
            pos_v = (pos_v + ((crate::CANVAS_SIZE / 2) as f64)).round();
        } else {
            pos_v = ((pos_v * -1.0) + (crate::CANVAS_SIZE / 2) as f64).round();
        }

        let pos_v = pos_v as i64;

        Ok(Ponto {
            id,
            r_asc,
            dec,
            plx,
            pos_h,
            pos_v,
            mag,
        })
    }
}
