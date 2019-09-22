mod kmeans;
mod plotter;
use kmeans::KlusterMeans;
use plotter::{draw, mean};

use rand::{thread_rng, Rng};

pub fn random_number<T>(bottom: T, up: T) -> T
where
    T: rand::distributions::uniform::SampleUniform,
{
    let mut random = thread_rng();
    random.gen_range(bottom, up)
}
fn main() {
    let points = vec![[0.0, 0.0], [2.0, 2.0], [4.0, 7.0]];
    // draw(vec![(0.0, 0.0), (2.0, 2.0), (4.0, 7.0)]);
    let mean = mean(vec![2.0, 2.0, 4.0, 7.0]);
    let kmeans = KlusterMeans::new(3, points);
    let result = kmeans.get_range_for_dimension(1);
    println!("Mean {:?}", kmeans.init_random_centroids());
}
