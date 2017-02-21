#![crate_name = "sudoku"]

pub fn print_sudoku(puzzle: &[u32]) {

    let (big_dim, check_small, small_dim) = calculate_squares(puzzle.len() as u32).unwrap();

    for (i, v) in puzzle.iter().enumerate() {
        print!("{} ", v);
        if check_small && (i + 1) % small_dim as usize == 0 {
            print!(" ");
        }
        if (i + 1) % big_dim as usize == 0 {
            println!("");
        }
        if check_small && (i + 1) % (small_dim * big_dim) as usize == 0 {
            println!("");
        }
    }
}

/// Returns a Some with the lenght of a given sudoku puzzle, a bool
/// which indicates if there are inner squares and the size of the small one.
/// Note: The value of the smaller one might be nonsense if the bool is false.
///
/// # Examples
///
/// ```
/// use sudoku::calculate_squares;
///
/// let sudoku = vec![1,2,3,4];
/// let (big_dim, check_small, small_dim) = calculate_squares(sudoku.len() as u32).unwrap();
/// ```
pub fn calculate_squares(puzzle: u32) -> Option<(u32, bool, u32)> {
    let big_dim = f64::sqrt(puzzle as f64) as u32;
    if big_dim * big_dim != puzzle as u32 {
        return None;
    }
    let small_dim = f64::sqrt(big_dim as f64) as u32;
    let check_inner_square = {
        small_dim * small_dim == big_dim
    };
    Some((big_dim, check_inner_square, small_dim))
}

pub fn check_if_unique(input: &[u32]) -> bool {
    let mut elements = vec![0;input.len()];
    let iter = input.into_iter().filter(|&x| *x != 0);
    for i in iter {
        if *i as usize >= elements.len() {
            let mut add = vec![0;*i as usize-elements.len()+1];
            elements.append(&mut add);
        }
        if elements[*i as usize] != 0 {
            return false;
        }
        elements[*i as usize] = *i;
    }
    true
}

fn check_if_valid_sudoku(sudoku: &[u32]) -> bool {
    let (big_dim, check_small, small_dim) = calculate_squares(sudoku.len() as u32).unwrap();
    for i in 0..big_dim {
        if !(check_if_unique(&get_row(&sudoku, i, big_dim)) &&
             check_if_unique(&get_col(&sudoku, i, big_dim))) {
            return false;
        }
        if check_small {
            if !check_if_unique(&get_sqr(&sudoku, i, big_dim, small_dim)) {
                return false;
            }
        }
    }
    true

}

fn get_row(sudoku: &[u32], row: u32, dim: u32) -> Vec<u32> {
    sudoku[(row * dim) as usize..(row * dim + dim) as usize].to_vec()
}
fn get_col(sudoku: &[u32], col: u32, dim: u32) -> Vec<u32> {
    (0..dim)
        .map(|x| sudoku[(col + x * dim) as usize])
        .collect::<Vec<u32>>()
}
fn get_sqr(sudoku: &[u32], square: u32, dim: u32, small_dim: u32) -> Vec<u32> {
    let mut output: Vec<u32> = Vec::with_capacity(dim as usize);
    let square_row = square / small_dim;
    let square_col = square % small_dim;

    for a in 0..small_dim {
        for b in 0..small_dim {
            output.push(sudoku[((square_row * small_dim * dim) + (small_dim * square_col) + b +
                         (a * dim)) as usize]);
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_pos() {
        let mut vec = vec![0, 0, 0, 0];
        assert!(check_if_unique(&vec));
        vec = vec![1, 2, 0, 3];
        assert!(check_if_unique(&vec));
        vec = vec![1];
        assert!(check_if_unique(&vec));
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_unique_neg() {
        let mut vec = vec![1, 1];
        assert!(check_if_unique(&vec));
        vec = vec![1, 2, 0, 1];
        assert!(check_if_unique(&vec));
        vec = vec![1, 4, 8, 0, 0, 4, 8];
        assert!(check_if_unique(&vec));
    }

    #[test]
    fn test_print() {
        print_sudoku(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        println!("");
        print_sudoku(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
    }

    #[test]
    fn calculate_squares_test_pos() {
        assert_eq!(calculate_squares(9).unwrap(), (3, false, 1));
        assert_eq!(calculate_squares(81).unwrap(), (9, true, 3));
        assert_eq!(calculate_squares(8), None)
    }

    fn test_sudoku() -> Vec<u32> {
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
    }

    #[test]
    fn get_row_test() {
        assert_eq!(get_row(&test_sudoku(), 0, 4), vec![1, 2, 3, 4]);
        assert_eq!(get_row(&test_sudoku(), 2, 4), vec![9, 10, 11, 12]);
        assert_eq!(get_row(&test_sudoku(), 3, 4), vec![13, 14, 15, 16]);
    }
    #[test]
    fn get_col_test() {
        assert_eq!(get_col(&test_sudoku(), 0, 4), vec![1, 5, 9, 13]);
        assert_eq!(get_col(&test_sudoku(), 2, 4), vec![3, 7, 11, 15]);
        assert_eq!(get_col(&test_sudoku(), 3, 4), vec![4, 8, 12, 16]);
    }
    #[test]
    fn get_sqr_test() {
        assert_eq!(get_sqr(&test_sudoku(), 0, 4, 2), vec![1, 2, 5, 6]);
        assert_eq!(get_sqr(&test_sudoku(), 1, 4, 2), vec![3, 4, 7, 8]);
        assert_eq!(get_sqr(&test_sudoku(), 2, 4, 2), vec![9, 10, 13, 14]);
        assert_eq!(get_sqr(&test_sudoku(), 3, 4, 2), vec![11, 12, 15, 16]);
    }

    #[test]
    fn valid_test_pos() {
        assert!(check_if_valid_sudoku(&test_sudoku()));
    }
    #[test]
    #[should_panic(expected = "assertion failed")]
    fn valid_test_neg() {
        let sudoku = vec![1, 2, 3, 3];
        assert!(check_if_valid_sudoku(&sudoku));
        let sudoku = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 1, 14, 15, 16]; //two 1 in col
        assert!(check_if_valid_sudoku(&sudoku));
        let sudoku = vec![1, 2, 2, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]; //two 2 in small square
        assert!(check_if_valid_sudoku(&sudoku));
        let sudoku = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 16, 15, 16]; //two 16 in row
        assert!(check_if_valid_sudoku(&sudoku));
    }
}