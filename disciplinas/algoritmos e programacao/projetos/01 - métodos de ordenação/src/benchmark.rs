use std::time::Instant;

pub fn measure_time_elapsed<F>(mut data: Vec<i32>, sort_fn: F) -> f32
where
    F: Fn(&mut [i32]),
{
    let start = Instant::now();
    sort_fn(&mut data);
    start.elapsed().as_secs_f32()
}