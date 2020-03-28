use plotters::prelude::*;

fn grafico(sec: Vec<crate::models::Ponto>) {
    // plot em gráfico
    let res = 2500;

    let root = BitMapBackend::new("scatter.png", (res, res)).into_drawing_area();

    let mut chart = ChartBuilder::on(&root)
        .caption("y=x^2", ("sans-serif", 50).into_font())
        .margin(20)
        .x_label_area_size(50)
        .y_label_area_size(50)
        .build_ranged(-100..crate::CANVAS_SIZE, -100..crate::CANVAS_SIZE)
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
