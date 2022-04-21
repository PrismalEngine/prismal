pub use xxhash_rust::xxh3::xxh3_64 as fast_hash_64;
pub use xxhash_rust::xxh3::xxh3_64_with_secret as fast_hash_with_secret_64;
pub use xxhash_rust::xxh3::xxh3_64_with_seed as fast_hash_with_seed_64;

pub use xxhash_rust::xxh3::xxh3_128 as fast_hash_128;
pub use xxhash_rust::xxh3::xxh3_128_with_secret as fast_hash_with_secret_128;
pub use xxhash_rust::xxh3::xxh3_128_with_seed as fast_hash_with_seed_128;

pub use xxhash_rust::const_xxh3::const_custom_default_secret as const_fast_hash_secret;

pub use xxhash_rust::const_xxh3::xxh3_64 as const_fast_hash_64;
pub use xxhash_rust::const_xxh3::xxh3_64_with_secret as const_fast_hash_with_secret_64;
pub use xxhash_rust::const_xxh3::xxh3_64_with_seed as const_fast_hash_with_seed_64;

pub use xxhash_rust::const_xxh3::xxh3_128 as const_fast_hash_128;
pub use xxhash_rust::const_xxh3::xxh3_128_with_secret as const_fast_hash_with_secret_128;
pub use xxhash_rust::const_xxh3::xxh3_128_with_seed as const_fast_hash_with_seed_128;
