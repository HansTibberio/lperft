use lperft::{perft_hash_multi, perft_hash_single, perft_multi, perft_single};

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
    #[arg(short = 't', long = "threads", default_value = "1")]
    threads: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Args = Args::parse();

    let board: Board = Board::from_str(&args.fen)?;

    if let Some(hash_size) = args.hash {
        if args.threads == 1 {
            let _ = perft_hash_single(&board, args.depth, hash_size);
        } else if args.threads >= 1 {
            let _ = perft_hash_multi(&board, args.depth, hash_size, args.threads);
        }
    } else if args.threads == 1 {
        let _ = perft_single(&board, args.depth);
    } else if args.threads >= 1 {
        let _ = perft_multi(&board, args.depth, args.threads);
    }

    Ok(())
}
