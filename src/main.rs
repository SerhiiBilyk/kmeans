mod kmeans;
mod utils;

use kmeans::KlusterMeans;

fn main() {
    let points = vec![
        [2.0, 2.0],
        [2.0, 4.0],
        [4.0, 4.0],
        [4.0, 2.0],
        [3.0, 2.0],
        [7.0, 7.0],
        [8.0, 6.0],
        [8.0, 8.0],
        [9.0, 7.0],
        [8.0, 2.0],
        [7.0, 2.0],
        [7.0, 2.5],
        [7.9, 1.5],
    ];

    let double = points.clone();
    let mut kmeans = KlusterMeans::new(3, points);
    kmeans.init_random_centroids();
    kmeans.run();

    //  draw(kmeans.result(), kmeans.to_tuples(double));
}
