pub mod perft_ht_mt;
pub mod perft_ht_st;
pub mod perft_mt;
pub mod perft_st;
pub mod table_mt;
pub mod table_st;

use crate::{
    perft_ht_mt::perft_hash_multi, perft_ht_st::perft_hash_single, perft_mt::perft_multi,
    perft_st::perft_single,
};
use clap::Parser;
use laura_core::Board;
use std::{error::Error, str::FromStr};

#[derive(Parser)]
#[command(version, about = "lperft - A blazingly fast perft tool", long_about = None)]
struct Args {
    /// FEN string representing the chess position.
    #[arg(
        short = 'f',
        long = "fen",
        default_value = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    )]
    fen: String,

    /// Search depth for node exploration.
    #[arg(short = 'd', long = "depth")]
    depth: usize,

    /// Transposition table size in MB (optional).
    #[arg(short = 'H', long = "hash")]
    hash: Option<usize>,

    /// Number of threads for parallel search.
    #[arg(short = 't', long = "thread", default_value = "1")]
    thread: usize,
}
fn main() -> Result<(), Box<dyn Error>> {
    let args: Args = Args::parse();

    let board: Board = Board::from_str(&args.fen)?;

    if let Some(hash_size) = args.hash {
        if args.thread == 1 {
            let _ = perft_hash_single(&board, args.depth, hash_size);
        } else if args.thread >= 1 {
            let _ = perft_hash_multi(&board, args.depth, hash_size, args.thread);
        }
    } else if args.thread == 1 {
        let _ = perft_single(&board, args.depth);
    } else if args.thread >= 1 {
        let _ = perft_multi(&board, args.depth, args.thread);
    }

    Ok(())
}
