use rand::SeedableRng;

const N: usize = 32;
pub struct SimpleRngSeed(pub [u8; N]);
pub struct SimpleRng(SimpleRngSeed);

impl Default for SimpleRngSeed {
    fn default() -> Self {
        SimpleRngSeed([0; N])
    }
}

impl AsMut<[u8]> for SimpleRngSeed {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl SeedableRng for SimpleRng {
    type Seed = SimpleRngSeed;

    fn from_seed(seed: Self::Seed) -> Self {
        SimpleRng(seed)
    }
}
