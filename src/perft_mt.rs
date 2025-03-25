// perft_mt.rs

//! Perft (performance test) without a hash table and multi-threaded execution.

use laura_core::{enumerate_legal_moves, Board, ALL_MOVES};
use rayon::{ThreadPool, ThreadPoolBuilder};
use smallvec::SmallVec;
use std::{
    sync::atomic::{AtomicUsize, Ordering},
    time::Instant,
};

#[inline(always)]
pub fn perft_multi(board: &Board, depth: usize, num_threads: usize) -> usize {
    let pool: ThreadPool = ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()
        .unwrap();

    let start: Instant = Instant::now();

    let total_nodes: usize = pool.install(|| {
        let mut moves: SmallVec<[laura_core::Move; 128]> = SmallVec::new();
        enumerate_legal_moves::<ALL_MOVES, _>(board, |mv| {
            moves.push(mv);
            true
        });

        let num_moves: usize = moves.len();
        if num_moves == 0 {
            return 0;
        }

        let chunk_size: usize = num_moves.div_ceil(num_threads);
        let total_nodes: AtomicUsize = AtomicUsize::new(0);

        rayon::scope(|s| {
            for chunk in moves.chunks(chunk_size) {
                let total_nodes_ref: &AtomicUsize = &total_nodes;
                let board_ref: &Board = board;

                s.spawn(move |_| {
                    let mut local_count: usize = 0;
                    for mv in chunk {
                        let board_res: Board = board_ref.make_move(*mv);
                        local_count += inner_perft::<false>(&board_res, depth - 1);
                        println!("{} -> {}", mv, local_count);
                    }
                    total_nodes_ref.fetch_add(local_count, Ordering::Relaxed);
                });
            }
        });

        total_nodes.load(Ordering::Relaxed)
    });

    let duration: std::time::Duration = start.elapsed();
    let nps: f64 = total_nodes as f64 / duration.as_secs_f64();
    println!("{total_nodes} nodes in {duration:?} -> {nps:.0} nodes/s");

    total_nodes
}

#[inline(always)]
fn inner_perft<const DIV: bool>(board: &Board, depth: usize) -> usize {
    if depth == 0 {
        return 1;
    }

    let mut total: usize = 0;

    if !DIV && depth <= 1 {
        enumerate_legal_moves::<ALL_MOVES, _>(board, |_| -> bool {
            total += 1;
            true
        });
        return total;
    }

    enumerate_legal_moves::<ALL_MOVES, _>(board, |mv| -> bool {
        let board_res: Board = board.make_move(mv);
        let nodes: usize = inner_perft::<false>(&board_res, depth - 1);
        total += nodes;

        true
    });

    total
}
