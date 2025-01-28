/// a wrapper over rand to allow mocking the random number generator
#[cfg(test)]
use mockall::automock;

use rand::RngCore;

#[allow(dead_code)]
pub struct RngWrapper;

#[cfg_attr(test, automock)]
impl RngWrapper {
    #[allow(dead_code)]
    pub fn next_u32() -> u32 {
        rand::rng().next_u32()
    }
}
