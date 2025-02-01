/// a wrapper over rand to allow mocking the random number generator
#[cfg(test)]
use mockall::automock;

use rand::{rngs::SmallRng, Rng, SeedableRng};

#[allow(dead_code)]
pub struct RngWrapper;

#[cfg_attr(test, automock)]
impl RngWrapper {
    #[allow(dead_code)]
    pub fn next_u32() -> u32 {
        let mut rng = SmallRng::from_os_rng();
        rng.random()
    }
}
