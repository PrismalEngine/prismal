use std::hash::BuildHasher;
use std::hash::Hasher;

#[derive(Default)]
pub struct IntHasher {
    value: u64,
}

impl Hasher for IntHasher {
    fn finish(&self) -> u64 {
        self.value
    }

    fn write(&mut self, _bytes: &[u8]) {
        panic!("Attempting to hash &[u8] using `IntHasher`");
    }

    fn write_u8(&mut self, i: u8) {
        self.value = i as u64;
    }

    fn write_u16(&mut self, i: u16) {
        self.value = i as u64;
    }

    fn write_u32(&mut self, i: u32) {
        self.value = i as u64;
    }

    fn write_u64(&mut self, i: u64) {
        self.value = i;
    }

    fn write_u128(&mut self, _: u128) {
        panic!("Attempting to hash u128 using `IntHasher`");
    }

    fn write_usize(&mut self, _: usize) {
        panic!("Attempting to hash usize using `IntHasher`");
    }
}

#[derive(Default)]
pub struct IntHasherBuilder;

impl BuildHasher for IntHasherBuilder {
    type Hasher = IntHasher;
    fn build_hasher(&self) -> Self::Hasher {
        IntHasher::default()
    }
}
