use crate::utils;
use utils::{distance, mean, random_number};

pub type Point = [f32; 2];

#[derive(Debug)]
pub struct Range {
    min: f32,
    max: f32,
}

const DIMENSIONALITY: usize = 2;

#[derive(Debug)]
pub struct KlusterMeans {
    klusters: i8,
    points: Vec<Point>,
    iterations: i32,
    centroid_assignments: Vec<i32>,
    pub centroids: Vec<Point>,
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
        let mut ranges: Vec<Range> = Vec::with_capacity(DIMENSIONALITY);
        for i in 0..DIMENSIONALITY {
            ranges.push(self.get_range_for_dimension(i))
        }
        ranges
    }

    pub fn init_random_centroids(&mut self) -> Vec<Point> {
        let dimension_ranges = self.get_all_dimension_ranges();
        let mut centroids: Vec<Point> = Vec::with_capacity(self.klusters as usize);
        for _ in 0..self.klusters {
            let mut point: Point = [0.0, 0.0];
            for dimension in 0..DIMENSIONALITY {
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
        let mut min_distance = 0.0;
        let mut assigned_centroid: Option<i32> = None;

        for index in 0..self.centroids.len() {
            let centroid = self.centroids[index];
            let distance_to_centroid = distance(self.points[point_index], centroid);
            println!("Index {:?}", self.centroids);

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
    pub fn assign_points_to_centroids(&mut self) -> bool {
        let mut was_any_reassigned = false;
        for index in 0..self.points.len() {
            let was_reassigned = self.assign_point_to_centroid(index);
            if was_reassigned == true {
                return true;
            }
        }
        false
    }
    pub fn get_points_for_centroid(&self, centroid_index: usize) -> Vec<Point> {
        let mut points: Vec<Point> = vec![];
        for index in 0..self.points.len() {
            let assignment = self.centroid_exist(index);
            if let Some(value) = assignment {
                if value == centroid_index as i32 {
                    points.push(self.points[index])
                }
            }
        }
        points
    }
    pub fn update_centroid_location(&mut self, index: usize) -> Point {
        let centroid_points = self.get_points_for_centroid(index);
        let mut centroid: Point = [0.0, 0.0];

        for dimension in 0..DIMENSIONALITY {
            centroid[dimension] = mean(
                centroid_points
                    .iter()
                    .map(|elem| elem[dimension])
                    .collect::<Vec<f32>>(),
            );
        }
        self.centroids[index] = centroid;
        centroid
    }
    pub fn update_centroid_locations(&mut self) {
        for index in 0..self.centroids.len() {
            self.update_centroid_location(index);
        }
    }
    pub fn run(&mut self) {
        let mut iteration = 0;
        loop {
            let did_assignments_change = self.assign_points_to_centroids();
            self.update_centroid_locations();
            iteration += 1;

            if iteration == 2 {
                break;
            }
        }
    }
    pub fn reset(&mut self) {
        self.centroid_assignments = vec![];
    }
}
