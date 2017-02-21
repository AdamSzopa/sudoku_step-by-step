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

fn unique_small_2(bench: &mut Bencher) {
    let vec = vec![1, 2, 3, 4];
    bench.iter(|| check_if_unique_2(&vec))
}
fn unique_medium_2(bench: &mut Bencher) {
    let vec = vec![9, 8, 7, 0, 6, 5, 4, 0, 3, 2, 1];
    bench.iter(|| check_if_unique_2(&vec))
}
fn unique_large_2(bench: &mut Bencher) {
    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 0, 0, 0, 0, 0, 10, 11, 12, 13, 14, 16, 19, 20, 0,
                   0, 44, 77, 0, 100];
    bench.iter(|| check_if_unique_2(&vec))
}

benchmark_group!(unique, unique_small, unique_medium, unique_large);
benchmark_group!(unique_2, unique_small_2, unique_medium_2, unique_large_2);
benchmark_main!(unique, unique_2);