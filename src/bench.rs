#[macro_use]
extern crate bencher;
extern crate sudoku;

use bencher::Bencher;
use sudoku::*;

fn unique_small(bench: &mut Bencher) {
    let vec = vec![1, 2, 3, 4];
    bench.iter(|| check_if_unique(&vec))
}
fn unique_medium(bench: &mut Bencher) {
    let vec = vec![9, 8, 7, 0, 6, 5, 4, 0, 3, 2, 1];
    bench.iter(|| check_if_unique(&vec))
}
fn unique_large(bench: &mut Bencher) {
    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 0, 0, 0, 0, 0, 10, 11, 12, 13, 14, 16, 19, 20, 0,
                   0, 44, 77, 0, 100];
    bench.iter(|| check_if_unique(&vec))
}

benchmark_group!(unique, unique_small, unique_medium, unique_large);
benchmark_main!(unique);