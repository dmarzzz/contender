use super::{SeedValue, Seeder};
use alloy::primitives::{keccak256, U256};
use rand::Rng;

/// Default seed generator, using a random 32-byte seed.
#[derive(Debug, Clone)]
pub struct RandSeed {
    seed: [u8; 32],
}

fn fill_bytes(seed: &[u8], target: &mut [u8; 32]) {
    if seed.len() < 32 {
        // right-pad with one-bytes
        target[0..seed.len()].copy_from_slice(seed);
        target[seed.len()..32].fill(0x01);
    } else {
        target.copy_from_slice(&seed[0..32]);
    }
}

impl RandSeed {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut seed = [0u8; 32];
        rng.fill(&mut seed);
        Self { seed }
    }

    pub fn from_bytes(seed_bytes: &[u8]) -> Self {
        let mut seed_arr = [0u8; 32];
        fill_bytes(seed_bytes, &mut seed_arr);
        Self { seed: seed_arr }
    }

    pub fn from_str(seed: &str) -> Self {
        RandSeed::from_bytes(seed.as_bytes())
    }

    pub fn from_u256(seed: U256) -> Self {
        Self {
            seed: seed.to_le_bytes(),
        }
    }
}

impl SeedValue for RandSeed {
    fn as_bytes(&self) -> &[u8] {
        &self.seed
    }

    fn as_u64(&self) -> u64 {
        u64::from_le_bytes(self.seed[0..8].try_into().unwrap())
    }

    fn as_u128(&self) -> u128 {
        u128::from_le_bytes(self.seed[0..16].try_into().unwrap())
    }

    fn as_u256(&self) -> U256 {
        U256::from_le_bytes::<32>(self.seed.try_into().unwrap())
    }
}

impl Seeder for RandSeed {
    fn seed_values(
        &self,
        amount: usize,
        min: Option<U256>,
        max: Option<U256>,
    ) -> Box<impl Iterator<Item = impl SeedValue>> {
        let min = min.unwrap_or(U256::ZERO);
        let max = max.unwrap_or(U256::MAX);
        let vals = (0..amount).map(move |i| {
            // generate random-looking value between min and max from seed
            let seed_num = self.as_u256() + U256::from(i);
            let val = keccak256(seed_num.as_le_slice());
            let val = U256::from_le_bytes(val.0);
            let val = val % (max - min) + min;
            RandSeed::from_u256(val)
        });
        Box::new(vals.to_owned())
    }
}

impl Default for RandSeed {
    fn default() -> Self {
        Self::new()
    }
}
