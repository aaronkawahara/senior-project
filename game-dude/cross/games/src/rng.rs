use rand::distributions::uniform::{SampleRange, SampleUniform};
pub use rand::{Rng, rngs::SmallRng, SeedableRng};

const RNG_SEED: [u8; 16] = [
    57, 15, 4, 218, 230, 117, 34, 242, 173, 21, 102, 234, 23, 225, 59,
    137,
    // 180, 233, 32, 108, 41, 189, 248, 144, 83, 48, 250, 211, 129, 61, 22, 137
];

pub static mut RNG: Option<SmallRng> = None;

pub fn init() {
    unsafe {
        if RNG.is_none() {
            RNG = Some(SmallRng::from_seed(RNG_SEED))
        }
    }
}

pub fn gen_range<T, R>(range: R) -> T
where 
    T: SampleUniform,
    R: SampleRange<T>,
{
    unsafe {
        if let Some(rng) = &mut RNG {
            rng.gen_range(range)
        } else {
            panic!("rng was not initialized")
        }
    }
}