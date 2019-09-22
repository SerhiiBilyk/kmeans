use rand::{thread_rng, Rng};

pub fn random_number<T>(bottom: T, up: T) -> T
where
    T: rand::distributions::uniform::SampleUniform,
{
    let mut random = thread_rng();
    random.gen_range(bottom, up)
}

#[derive(Debug)]
pub struct Range {
    min: f32,
    max: f32,
}

type Point = [f32; 2];

#[derive(Debug)]
pub struct KlusterMeans {
    klusters: i8,
    points: Vec<Point>,
    iterations: i32,
}

impl KlusterMeans {
    pub fn new(klusters: i8, points: Vec<Point>) -> KlusterMeans {
        KlusterMeans {
            klusters,
            points,
            iterations: 0,
        }
    }
    pub fn get_dimensionality(&self) -> usize {
        self.points[0].len()
    }
    pub fn get_range_for_dimension(&self, dimension: usize) -> Range {
        let values = self
            .points
            .iter()
            .fold(Range { min: 0.0, max: 0.0 }, |acc, elem| {
                let Range { min, max } = acc;
                let value = elem[dimension];

                Range {
                    min: min.min(value),
                    max: max.max(value),
                }
            });
        values
    }

    pub fn get_all_dimension_ranges(&self) -> Vec<Range> {
        let dimensionality = self.get_dimensionality();
        let mut ranges: Vec<Range> = Vec::with_capacity(dimensionality);
        for i in 0..dimensionality {
            ranges.push(self.get_range_for_dimension(i))
        }
        ranges
    }

    pub fn init_random_centroids(&self) -> Vec<Point> {
        let dimensionality = self.get_dimensionality();
        let dimension_ranges = self.get_all_dimension_ranges();
        let mut centroids: Vec<Point> = Vec::with_capacity(self.klusters as usize);
        for kluster in 0..self.klusters {
            let mut point: Point = [0.0, 0.0];
            for dimension in 0..dimensionality {
                let Range { min, max } = dimension_ranges[dimension];
                point[dimension] = random_number::<f32>(min, max);
            }
            centroids.push(point)
        }
        centroids
    }
    pub fn reset(&mut self) {}
}
