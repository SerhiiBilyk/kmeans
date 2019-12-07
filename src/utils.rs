use rand::{thread_rng, Rng};

pub fn mean(numbers: Vec<f32>) -> f32 {
    numbers.iter().fold(0.0, |acc, value| acc + value) / numbers.len() as f32
}

pub fn distance(a: [f32; 2], b: [f32; 2]) -> f64 {
    f64::from((b[0] - a[0]).powf(2.0) + (b[1] - a[1]).powf(2.0))
}

pub fn random_number<T>(bottom: T, up: T) -> T
where
    T: rand::distributions::uniform::SampleUniform,
{
    let mut random = thread_rng();
    random.gen_range(bottom, up)
}
