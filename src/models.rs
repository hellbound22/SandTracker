#[derive(Debug, Clone)]
pub struct Ponto {
    id: i32,        // Identificador Hipparcos
    pub r_asc: f64, // Ascensão Reta
    pub dec: f64,   // Declinação
    plx: f64,       // Paralaxe
    pub pos_h: i64, // Posição na horizontal normalizada (0-21600)
    pub pos_v: i64, // Posição na vertical normalizada (0-21600)
}

impl Ponto {
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
        /*
                let mut pos_v = ((dec * 180.0 / 3.1416) * 60.0).round() as i64; // há negativos

                let mut pos_v = {
                    if dec < 0.0 {
                        (((dec * 180.0 / 3.1416) * 60.0).round() as i64) * -1 + (crate::CANVAS_SIZE / 2)
                    } else {
                        ((dec * 180.0 / 3.1416) * 60.0).round() as i64
                    }
                };
        */
        let pos_h = (r_asc.to_degrees() * 60.0).round() as i64;

        let mut pos_v = (dec.to_degrees().abs() * 60.0);
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
        })
    }
}
