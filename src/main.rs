mod caches;
mod list;
mod test;
mod traits;
mod utils;

use test::{TestConfig, run_test};
use plotters::prelude::*;
const OUT_FILE_NAME: &str = "./grahps/histogram.png";

fn main() {
    let _ = main_test();
}

fn main_test() -> Result<(), Box<dyn std::error::Error>> {
    let config = TestConfig {
        num_samples: 1000,
        num_accesses: 10000,
        skew: 1.0,
    };

    let mut no_cache = caches::None::new();
    let mut rand_cache = caches::Rand::new();
    let mut fifo_cache = caches::Fifo::new();
    let mut lru_cache = caches::LRU::new();
    let mut freq_cache = caches::Freq::new();

    // run_test(&mut no_cache, &config);
    // run_test(&mut rand_cache, &config);
    // run_test(&mut fifo_cache, &config);    
    // run_test(&mut lru_cache, &config);
    // run_test(&mut freq_cache, &config);

    //base code for histogram taken from: https://github.com/plotters-rs/plotters/blob/master/plotters/examples/histogram.rs
    let root = BitMapBackend::new(OUT_FILE_NAME, (640, 480)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5)
        .caption("Histogram Test", ("sans-serif", 50.0))
        .build_cartesian_2d((0u32..10u32).into_segmented(), 0u32..10u32)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .bold_line_style(WHITE.mix(0.3))
        .y_desc("Count")
        .x_desc("Bucket")
        .axis_desc_style(("sans-serif", 15))
        .draw()?;

    let data = [
        0u32, 1, 1, 1, 4, 2, 5, 7, 8, 6, 4, 2, 1, 8, 3, 3, 3, 4, 4, 3, 3, 3,
    ];

    chart.draw_series(
        Histogram::vertical(&chart)
            .style(RED.mix(0.5).filled())
            .data(data.iter().map(|x: &u32| (*x, 1))),
    )?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", OUT_FILE_NAME);

    Ok(())
}
