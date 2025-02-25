use crate::table_m::HashTable;
use laura_core::{enumerate_legal_moves, Board, ALL_MOVES};
use rayon::{ThreadPool, ThreadPoolBuilder};
use smallvec::SmallVec;
use std::{
    sync::atomic::{AtomicUsize, Ordering},
    time::Instant,
};

#[inline(always)]
pub fn perft_hash_multi(board: &Board, depth: usize, size: usize, num_threads: usize) -> usize {
    let mut table: HashTable = HashTable::default();
    table.resize(size);
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
                let table_ref: &HashTable = &table;

                s.spawn(move |_| {
                    let mut local_count: usize = 0;
                    for mv in chunk {
                        let board_res: Board = board_ref.make_move(*mv);
                        local_count += inner_perft::<false>(&board_res, depth - 1, table_ref);
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

    let used_slots: usize = table
        .table
        .iter()
        .filter(|entry| entry.zobrist.load(Ordering::Relaxed) != 0)
        .count();
    let usage: f64 = (used_slots as f64 / table.table.len() as f64) * 100.0;
    println!("Hash Table Usage: {:.2}%", usage);

    total_nodes
}

#[inline(always)]
fn inner_perft<const DIV: bool>(board: &Board, depth: usize, table: &HashTable) -> usize {
    if depth == 0 {
        return 1;
    }

    if depth <= 1 {
        let mut leaf_nodes: usize = 0;
        enumerate_legal_moves::<ALL_MOVES, _>(board, |_| -> bool {
            leaf_nodes += 1;
            true
        });
        return leaf_nodes;
    }

    if let Some(entry) = table.probe(board.zobrist) {
        if entry.get_depth() == depth {
            return entry.get_nodes();
        }
    }

    let mut total: usize = 0;

    enumerate_legal_moves::<ALL_MOVES, _>(board, |mv| -> bool {
        let board_res: Board = board.make_move(mv);
        let nodes: usize = inner_perft::<false>(&board_res, depth - 1, table);
        total += nodes;

        true
    });

    table.add(board.zobrist, total as u64, depth as u8);

    total
}
