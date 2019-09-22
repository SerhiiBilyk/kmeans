mod kmeans;
mod plotter;
mod utils;

use kmeans::KlusterMeans;
use plotter::{draw, mean};

fn main() {
    let points = vec![[0.0, 0.0], [2.0, 2.0], [4.0, 7.0]];
    // draw(vec![(0.0, 0.0), (2.0, 2.0), (4.0, 7.0)]);
    let mean = mean(vec![2.0, 2.0, 4.0, 7.0]);
    let mut kmeans = KlusterMeans::new(3, points);
    let result = kmeans.get_range_for_dimension(1);
    println!("Mean {:?}", kmeans.init_random_centroids());
}
