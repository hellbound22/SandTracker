use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

mod engine;
mod models;

const DEFAULT_STAR_SIZE: f32 = 5.0;
const CANVAS_SIZE: i64 = 360 * 60;
const FOV_SIZE: i64 = 5 * 60;
const STEP: i64 = (0.2 * 60.0) as i64;

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

    let mut canvas: HashMap<(i64, i64), models::Ponto> = HashMap::new();

    println!(
        "Tamanho do campo de visão (em arcminutos): {0} x {0}",
        FOV_SIZE
    );

    for star in &sec {
        canvas.insert(star.coord(), star.clone());
        //dbg!(star);
        //dbg!(star.mag());
        //dbg!(star.size());
    }

    let canvas = models::Canvas {
        size: CANVAS_SIZE,
        map: canvas,
    };

    println!(
        "Tamanho do canvas(espaço de plot / em arcminutos): {0} x {0}",
        canvas.size
    );

    let state = models::App::new(canvas);

    engine::start(state);
}
