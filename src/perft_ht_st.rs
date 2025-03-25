// perft_ht_st.rs

//! Perft (performance test) with a hash table and single-threaded execution.

use crate::table_st::{HashEntry, HashTable};
use laura_core::{enumerate_legal_moves, Board, ALL_MOVES};

#[inline(always)]
pub fn perft_hash_single(board: &Board, depth: usize, size: usize) -> usize {
    let mut table: HashTable = HashTable::default();
    table.resize(size);
    let start: std::time::Instant = std::time::Instant::now();
    let total_nodes: usize = inner_perft::<true>(board, depth, &mut table);
    let duration: std::time::Duration = start.elapsed();
    let nps: f64 = total_nodes as f64 / duration.as_secs_f64();
    println!("{total_nodes} nodes in {duration:?} -> {nps:.0} nodes/s");

    let used_slots: usize = table
        .table
        .iter()
        .filter(|entry| entry.zobrist != 0)
        .count();
    let usage: f64 = (used_slots as f64 / table.table.len() as f64) * 100.0;
    println!("Hash Table Usage: {:.2}%", usage);

    total_nodes
}

#[inline(always)]
#[allow(unused_assignments)]
fn inner_perft<const DIV: bool>(board: &Board, depth: usize, table: &mut HashTable) -> usize {
    if depth <= 1 {
        let mut leaf_nodes: usize = 0;
        enumerate_legal_moves::<ALL_MOVES, _>(board, |_| -> bool {
            leaf_nodes += 1;
            true
        });
        return leaf_nodes;
    }

    let entry: HashEntry = table.probe(board.zobrist);
    if entry.get_depth() == depth && entry.get_zobrist() == board.zobrist.0 as usize {
        return entry.get_nodes();
    }

    let mut total: usize = 0;

    enumerate_legal_moves::<ALL_MOVES, _>(board, |mv| -> bool {
        let board_res: Board = board.make_move(mv);
        let nodes: usize = inner_perft::<false>(&board_res, depth - 1, table);
        total += nodes;
        if DIV {
            println!("{} -> {}", mv, nodes);
        }

        true
    });

    table.add(board.zobrist, total as u64, depth as u8);

    total
}
