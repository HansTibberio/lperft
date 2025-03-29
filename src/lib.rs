#![doc = include_str!("../README.md")]

mod perft_ht_mt;
mod perft_ht_st;
mod perft_mt;
mod perft_st;
mod table_mt;
mod table_st;

pub use perft_ht_mt::*;
pub use perft_ht_st::*;
pub use perft_mt::*;
pub use perft_st::*;
