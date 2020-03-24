use plotters::prelude::*;

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

mod models;

const CANVAS_SIZE: i64 = 360 * 60;
const FOV_SIZE: i64 = 5 * 60;

pub struct Canvas {
    size: i64,
    map: HashMap<(i64, i64), Option<models::Ponto>>,
}

impl Canvas {
    pub fn get_vec(&self, h: i64, v: i64) -> Vec<bool> {
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

fn main() {
    let mut dataset = String::new();
    {
        let mut file_dataset = File::open("./dataset/hip2.dat").unwrap();
        file_dataset.read_to_string(&mut dataset).unwrap();
    }

    let sec: Vec<models::Ponto>;
    {
        let mut vc: Vec<models::Ponto> = Vec::new();

        for linha in dataset.split_terminator('\n') {
            vc.push(models::Ponto::new_from_line(linha).unwrap())
        }
        sec = vc.get(..).unwrap().to_vec();
    }

    let mut canvas: HashMap<(i64, i64), Option<models::Ponto>> = HashMap::new();

    println!(
        "Tamanho do campo de visão (em arcminutos): {0} x {0}",
        FOV_SIZE
    );

    for star in &sec {
        canvas.insert(star.coord(), Some(star.clone()));
    }

    let canvas = Canvas {
        size: CANVAS_SIZE,
        map: canvas,
    };

    println!(
        "Tamanho do canvas(espaço de plot / em arcminutos): {0} x {0}",
        canvas.size
    );

    // plot em gráfico
    let res = 2500;

    let root = BitMapBackend::new("scatter.png", (res, res)).into_drawing_area();

    let mut chart = ChartBuilder::on(&root)
        .caption("y=x^2", ("sans-serif", 50).into_font())
        .margin(20)
        .x_label_area_size(50)
        .y_label_area_size(50)
        .build_ranged(-100..CANVAS_SIZE, -100..CANVAS_SIZE)
        .unwrap();

    root.fill(&WHITE).unwrap();

    chart.configure_mesh().draw().unwrap();

    chart.draw_series(PointSeries::<_, _, Circle<_, _>, _>::new(
        sec.iter().map(|x| (x.pos_h, x.pos_v)),
        1,
        ShapeStyle {
            color: BLACK.to_rgba(),
            filled: true,
            stroke_width: 1,
        },
    ));
}
