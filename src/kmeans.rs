use crate::utils;
use utils::{distance, random_number};

type Point = [f32; 2];

#[derive(Debug)]
pub struct Range {
    min: f32,
    max: f32,
}

#[derive(Debug)]
pub struct KlusterMeans {
    klusters: i8,
    points: Vec<Point>,
    iterations: i32,
    centroid_assignments: Vec<i32>,
    centroids: Vec<Point>,
}

impl KlusterMeans {
    pub fn new(klusters: i8, points: Vec<Point>) -> KlusterMeans {
        KlusterMeans {
            klusters,
            points,
            iterations: 0,
            centroid_assignments: vec![],
            centroids: vec![],
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

    pub fn init_random_centroids(&mut self) -> Vec<Point> {
        let dimensionality = self.get_dimensionality();
        let dimension_ranges = self.get_all_dimension_ranges();
        let mut centroids: Vec<Point> = Vec::with_capacity(self.klusters as usize);
        for _ in 0..self.klusters {
            let mut point: Point = [0.0, 0.0];
            for dimension in 0..dimensionality {
                let Range { min, max } = dimension_ranges[dimension];
                point[dimension] = random_number::<f32>(min, max);
            }
            centroids.push(point)
        }
        self.centroids = centroids.clone();

        centroids
    }

    pub fn centroid_exist(&self, index: usize) -> Option<i32> {
        match self.centroid_assignments.get(index) {
            Some(value) => Some(*value),
            None => None,
        }
    }

    pub fn assign_point_to_centroid(&mut self, point_index: usize) -> bool {
        let last_assigned = self.centroid_exist(point_index);
        let point = self.points[point_index];
        let mut min_distance = 0.0;
        let mut assigned_centroid: Option<i32> = None;

        for index in 0..self.centroids.len() {
            let centroid = self.centroids[index];
            let distance_to_centroid = distance(point, centroid);
            if min_distance == 0.0 || distance_to_centroid < min_distance {
                min_distance = distance_to_centroid;
                assigned_centroid = Some(index as i32);
            }
        }

        match (assigned_centroid, last_assigned) {
            (Some(centroid), Some(last)) => {
                self.centroid_assignments[point_index] = centroid;
                last != centroid
            }
            _ => false,
        };

        false
    }
    pub fn reset(&mut self) {
        self.centroid_assignments = vec![];
    }
}
