pub mod perft_h_m;
pub mod perft_h_s;
pub mod perft_n_m;
pub mod perft_n_s;
pub mod table_m;
pub mod table_s;

use crate::{
    perft_h_m::perft_hash_multi, perft_h_s::perft_hash_single, perft_n_m::perft_nothash_multi,
    perft_n_s::perft_nothash_single,
};
use clap::Parser;
use laura_core::Board;
use std::{error::Error, str::FromStr};

#[derive(Parser)]
#[command(version, about = "lperft - The Faster Perft Tool", long_about = None)]
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
        let _ = perft_nothash_single(&board, args.depth);
    } else if args.thread >= 1 {
        let _ = perft_nothash_multi(&board, args.depth, args.thread);
    }

    Ok(())
}
