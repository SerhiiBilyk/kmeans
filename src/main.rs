mod kmeans;
mod plotter;
mod utils;

use kmeans::{KlusterMeans, Point};
use plotter::draw;
use utils::mean;

fn main() {
    let points = vec![
        [2.0, 2.0],
        [2.0, 4.0],
        [4.0, 4.0],
        [4.0, 2.0],
        [3.0, 2.0],
        [4.0, 6.0],
        [7.0, 7.0],
        [8.0, 6.0],
        [8.0, 8.0],
        [9.0, 7.0],
        [8.0, 2.0],
        [7.0, 2.0],
        [7.0, 2.5],
        [7.9, 1.5],
    ];
    // draw(vec![(0.0, 0.0), (2.0, 2.0), (4.0, 7.0)]);

    let double = points.clone();
    let mut kmeans = KlusterMeans::new(3, points);
    let result = kmeans.init_random_centroids();
    kmeans.run();
    println!("kmeans.result() {:?}", kmeans.result());

    draw(kmeans.result(), kmeans.to_tuples(double));
}
