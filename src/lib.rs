#![crate_name = "sudoku"]

pub fn print_sudoku(puzzle: &[u32]) {

    let (big_dim, check_small, small_dim) = calculate_squares(&puzzle).unwrap();

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

#[test]
fn test_print(){
    print_sudoku(&vec![1,2,3,4,5,6,7,8,9]);
    println!("");
    print_sudoku(&vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16]);
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
/// let (big_dim, check_small, small_dim) = calculate_squares(&sudoku).unwrap();
/// ```
pub fn calculate_squares(puzzle: &[u32]) -> Option<(u32, bool, u32)> {
    let big_dim = f64::sqrt(puzzle.len() as f64) as u32;
    if big_dim * big_dim != puzzle.len() as u32 {
        return None;
    }
    let small_dim = f64::sqrt(big_dim as f64) as u32;
    let check_inner_square = {
        small_dim * small_dim == big_dim
    };
    Some((big_dim, check_inner_square, small_dim))
}

fn check_if_unique(input: &[u32]) -> bool{
    true
}

#[test]
fn test_unique(){
    let mut vec = vec![0,0,0,0];
    assert!(check_if_unique(&vec));
    vec = vec![1,2,0,3];
    assert!(check_if_unique(&vec));
    vec = vec![1];
    assert!(check_if_unique(&vec));
}

