use laura_core::{enumerate_legal_moves, Board, ALL_MOVES};

#[inline(always)]
pub fn perft_nothash_single(board: &Board, depth: usize) -> usize {
    let start: std::time::Instant = std::time::Instant::now();
    let total_nodes: usize = inner_perft::<true>(board, depth);
    let duration: std::time::Duration = start.elapsed();

    let nps: f64 = total_nodes as f64 / duration.as_secs_f64();
    println!("{total_nodes} nodes in {duration:?} -> {nps:.0} nodes/s");

    total_nodes
}

#[inline(always)]
#[allow(unused_assignments)]
fn inner_perft<const DIV: bool>(board: &Board, depth: usize) -> usize {
    let mut total: usize = 0;

    if !DIV && depth <= 1 {
        enumerate_legal_moves::<ALL_MOVES, _>(board, |_| -> bool {
            total += 1;
            true
        });
        return total;
    }

    enumerate_legal_moves::<ALL_MOVES, _>(board, |mv| -> bool {
        let mut nodes: usize = 0;
        if DIV && depth == 1 {
            nodes = 1;
        } else {
            let board_res: Board = board.make_move(mv);
            nodes = if depth == 1 {
                1
            } else {
                inner_perft::<false>(&board_res, depth - 1)
            };
        }

        total += nodes;

        if DIV && nodes > 0 {
            println!("{} -> {}", mv, nodes);
        }

        true
    });

    total
}
