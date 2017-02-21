pub fn print_sudoku(puzzle: &[u32]) {
    //how do I know how big is the square?
    //does it have inner squares?
}

fn calculate_squares(puzzle: &[u32]) -> Option<(u32, bool, u32)> {
    let big_dim = f64::sqrt(puzzle.len() as f64) as u32;
    if big_dim * big_dim != puzzle.len() as u32 {
        return None;
    }
    let small_dim = f64::sqrt(big_dim as f64) as u32;
    let check_inner_square = {
        small_dim * small_dim == big_dim
    };
    Some((big_dim, check_inner_square,small_dim))
}
