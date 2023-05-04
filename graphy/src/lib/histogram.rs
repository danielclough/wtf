use ndarray::prelude::*;
use ndarray_rand::RandomExt;
use ndarray_rand::rand_distr::StandardNormal;
use ndarray_rand::rand as rand;
use ndarray_stats::HistogramExt;
use ndarray_stats::histogram::{strategies::Sqrt, GridBuilder};
use noisy_float::types::{N64, n64};

pub fn main() {
    // Histogram
    
    let arr17 = Array::<f64, _>::random_using((10000, 2), StandardNormal, &mut rand::thread_rng());

    let data = arr17.mapv(|e| n64(e));
    let grid = GridBuilder::<Sqrt<N64>>::from_array(&data).unwrap().build();
    let histogram = data.histogram(grid);

    let histogram_matrix = histogram.counts();
    
    let data = histogram_matrix.sum_axis(Axis(0));
    let his_data: Vec<(f32, f32)> = data.iter().enumerate().map(|(e, i)| (e as f32, *i as f32) ).collect();
    
    let file = std::fs::File::create("standard_normal_hist.svg").unwrap(); // create file on disk
    
    let mut graph = poloto::plot("Histogram", "x", "y"); // create graph
    graph.histogram("Stand.Norm.Dist.", his_data).ymarker(0);
    
    graph.simple_theme(poloto::upgrade_write(file));
}