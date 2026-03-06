use rand::RngExt;

#[derive(Debug, Clone, Copy)]
pub enum InputPattern {
    Random,
    Ascending,
    Descending,
}

pub fn generate_vec(size: usize, pattern: InputPattern) -> Vec<i32> {
    match pattern {
        InputPattern::Random => {
            let mut rng = rand::rng();
            (0..size).map(|_| rng.random_range(0..100000)).collect()
        }

        InputPattern::Ascending => {
            (0..size as i32).collect()
        }

        InputPattern::Descending => {
            (0..size as i32).rev().collect()
        }
    }
}