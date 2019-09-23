mod kmeans;
mod plotter;
mod utils;

use kmeans::{KlusterMeans, Point};
use plotter::draw;
use utils::mean;

fn main() {
    let points = vec![
        [1.0, 2.0],
        [2.0, 3.0],
        [2.0, 5.0],
        [1.0, 6.0],
        [4.0, 6.0],
        [3.0, 5.0],
        [2.0, 4.0],
        // [4.0, 3.0],
        // [5.0, 2.0],
        // [6.0, 9.0],
        // [4.0, 4.0],
        // [3.0, 3.0],
        // [8.0, 6.0],
        // [7.0, 5.0],
        // [9.0, 6.0],
        // [9.0, 7.0],
        // [8.0, 8.0],
        // [7.0, 9.0],
        // [11.0, 3.0],
        // [11.0, 2.0],
        // [9.0, 9.0],
        // [7.0, 8.0],
        // [6.0, 8.0],
        // [12.0, 2.0],
        // [14.0, 3.0],
        // [15.0, 1.0],
        // [15.0, 4.0],
        // [14.0, 2.0],
        // [13.0, 1.0],
        // [16.0, 4.0],
    ];
    // draw(vec![(0.0, 0.0), (2.0, 2.0), (4.0, 7.0)]);

    let mut kmeans = KlusterMeans::new(3, points);
    kmeans.init_random_centroids();
    kmeans.run();
    println!("Mean {:?}", kmeans.centroids);
}
