use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

mod models;

const CANVAS_SIZE: f64 = 360.0 * 60.0;
const FOV_SIZE: f64 = 5.0 * 60.0;

fn main() {
    let mut file_dataset = File::open("./dataset/hip2.dat").unwrap();
    let mut dataset = String::new();
    file_dataset.read_to_string(&mut dataset).unwrap();

    let mut vc: Vec<models::Ponto> = Vec::new();

    for linha in dataset.split_terminator('\n') {
        vc.push(models::Ponto::new_from_line(linha).unwrap())
    }

    let sec = vc.get(..100).unwrap();

    //let mut canvas: HashMap<(i64, i64), bool> = HashMap::new();
    let mut v_canvas = vec![false; CANVAS_SIZE as usize];
    let mut canvas = vec![v_canvas.clone(); CANVAS_SIZE as usize];

    println!(
        "Tamanho do canvas(espaço de plot / em arcminutos): {0} x {0}",
        CANVAS_SIZE
    );
    println!(
        "Tamanho do campo de visão (em arcminutos): {0} x {0}",
        FOV_SIZE
    );

    println!("======Posição em arcminuto======");
    for star in sec {
        // Posição em angulo
        canvas[star.pos_h as usize][star.pos_v as usize] = true;
        println!("{}, {}", star.pos_h, star.pos_v);
    }
    dbg!(&canvas[18]);
}
