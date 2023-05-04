use ndarray::prelude::*;
use ndarray_rand::rand as rand;
use ndarray_rand::RandomExt;
use ndarray_rand::rand_distr::StandardNormal;


pub fn main() {
    // Scatter Plot

    let arr18 = Array::<f64, _>::random_using((300, 2), StandardNormal, &mut rand::thread_rng());

    let data:  Vec<(f64, f64)> = arr18.axis_iter(Axis(0)).map(|e| {
        let v = e.to_vec();
        (v[0], v[1])
    }).collect();

    let x_line = [[-3,0], [3,0]];
    let y_line = [[0,-3], [0, 3]];
    
    let file = std::fs::File::create("standard_normal_scatter.svg").unwrap(); // create file on disk
    
    let mut graph = poloto::plot("Scatter Plot", "x", "y"); // create graph
    graph.line("", &x_line);
    graph.line("", &y_line);
    graph.scatter("Stand.Norm.Dist.", data).ymarker(0);
    
    graph.simple_theme(poloto::upgrade_write(file));   
}