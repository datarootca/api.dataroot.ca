pub mod response;
pub mod validator;

#[allow(unused)]
use rand::distributions::{Alphanumeric, DistString};
#[cfg(test)]
pub fn random_string(length: usize) -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), length)
}