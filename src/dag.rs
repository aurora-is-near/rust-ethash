use alloc::vec::Vec;
use core::marker::PhantomData;
use ethereum_types::{H256, H64, U256};

pub trait Patch {
    fn epoch_length() -> U256;
}

pub struct EthereumPatch;
impl Patch for EthereumPatch {
    fn epoch_length() -> U256 {
        U256::from(30000)
    }
}

pub struct LightDAG<P: Patch> {
    epoch: u64,
    cache: Vec<u8>,
    #[allow(dead_code)]
    cache_size: u64,
    full_size: u64,
    _marker: PhantomData<P>,
}

impl<P: Patch> LightDAG<P> {
    pub fn new(number: U256) -> Self {
        let epoch = (number / P::epoch_length()).as_u64();
        let cache_size = crate::get_cache_size(epoch);
        let full_size = crate::get_full_size(epoch);
        let seed = crate::get_seedhash(epoch);

        let mut cache: Vec<u8> = Vec::with_capacity(cache_size as usize);
        cache.resize(cache_size as usize, 0);
        crate::make_cache(&mut cache, seed);

        Self {
            cache,
            cache_size,
            full_size,
            epoch,
            _marker: PhantomData,
        }
    }

    pub fn hashimoto(&self, hash: H256, nonce: H64) -> (H256, H256) {
        crate::hashimoto_light(hash, nonce, self.full_size, &self.cache)
    }

    pub fn is_valid_for(&self, number: U256) -> bool {
        (number / P::epoch_length()).as_u64() == self.epoch
    }
}
