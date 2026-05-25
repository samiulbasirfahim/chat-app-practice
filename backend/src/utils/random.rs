use rand::distributions::Alphanumeric;
use rand::{self, Rng};

pub fn generate_random_username() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(14)
        .map(char::from)
        .collect()
}
