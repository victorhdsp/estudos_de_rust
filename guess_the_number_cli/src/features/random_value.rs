use rand::Rng;

pub fn generate () -> u32 {
    let mut rng = rand::thread_rng();
    let random_number: u32 = rng.gen_range(0..=100);
    return random_number;
}