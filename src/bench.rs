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

fn possible_small(bench: &mut Bencher) {
    let vec = vec![1, 2, 3, 4];
    bench.iter(|| check_if_possible(&vec, 3))
}
fn possible_medium(bench: &mut Bencher) {
    let vec = vec![9, 8, 7, 0, 6, 5, 4, 0, 3, 2, 1];
    bench.iter(|| check_if_possible(&vec, 3))
}
fn possible_large(bench: &mut Bencher) {
    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 0, 0, 0, 0, 0, 10, 11, 12, 13, 14, 16, 19, 20, 0,
                   0, 44, 77, 0, 100];
    bench.iter(|| check_if_possible(&vec, 20))
}

fn solve_small_simple(bench: &mut Bencher) {
    let vec = vec![0;25];
    bench.iter(|| solve(&vec))
}

fn solve_normal_simple(bench: &mut Bencher) {
    let vec = vec![0;81];
    bench.iter(|| solve(&vec))
}

fn solve_normal(bench: &mut Bencher) {
    let vec = vec![0, 0, 7, 0, 2, 0, 0, 0, 3, 8, 0, 5, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                   0, 0, 6, 0, 1, 5, 8, 0, 0, 0, 0, 3, 0, 0, 6, 0, 0, 0, 1, 7, 0, 0, 0, 0, 9, 0,
                   0, 0, 2, 9, 0, 0, 0, 6, 0, 0, 0, 3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
                   8, 0, 7];
    bench.iter(|| solve(&vec))
}
fn solve_small_simple_threads(bench: &mut Bencher) {
    let vec = vec![0;25];
    bench.iter(|| solve_threads(&vec))
}

fn solve_normal_simple_threads(bench: &mut Bencher) {
    let vec = vec![0;81];
    bench.iter(|| solve_threads(&vec))
}

fn solve_normal_threads(bench: &mut Bencher) {
    let vec = vec![0, 0, 7, 0, 2, 0, 0, 0, 3, 8, 0, 5, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                   0, 0, 6, 0, 1, 5, 8, 0, 0, 0, 0, 3, 0, 0, 6, 0, 0, 0, 1, 7, 0, 0, 0, 0, 9, 0,
                   0, 0, 2, 9, 0, 0, 0, 6, 0, 0, 0, 3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
                   8, 0, 7];
    bench.iter(|| solve_threads(&vec))
}
fn solve_small_simple_cross(bench: &mut Bencher) {
    let vec = vec![0;25];
    bench.iter(|| solve_cross(&vec))
}

fn solve_normal_simple_cross(bench: &mut Bencher) {
    let vec = vec![0;81];
    bench.iter(|| solve_cross(&vec))
}

fn solve_normal_cross(bench: &mut Bencher) {
    let vec = vec![0, 0, 7, 0, 2, 0, 0, 0, 3, 8, 0, 5, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                   0, 0, 6, 0, 1, 5, 8, 0, 0, 0, 0, 3, 0, 0, 6, 0, 0, 0, 1, 7, 0, 0, 0, 0, 9, 0,
                   0, 0, 2, 9, 0, 0, 0, 6, 0, 0, 0, 3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
                   8, 0, 7];
    bench.iter(|| solve_cross(&vec))
}

benchmark_group!(unique, unique_small, unique_medium, unique_large);
benchmark_group!(possible, possible_small, possible_medium, possible_large);
benchmark_group!(solver,
                 solve_small_simple,
                 solve_normal_simple,
                 solve_normal);
benchmark_group!(solver_threads,
                 solve_small_simple_threads,
                 solve_normal_simple_threads,
                 solve_normal_threads);
benchmark_group!(solver_cross,
                 solve_small_simple_cross,
                 solve_normal_simple_cross,
                 solve_normal_cross);
benchmark_main!(unique, possible, solver, solver_threads, solver_cross);