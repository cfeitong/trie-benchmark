pub mod naive_ac;
pub mod naive_trie;

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
