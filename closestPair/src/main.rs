mod par_dac;
mod dac;
mod grid;
use crate::par_dac::par_closest_distance;
use crate::dac::seq_closest_distance;
use crate::grid::*;
use rand::Rng;
use std::time::{Duration, Instant};

type Point = (i32, i32);
const A: i32 = 10;
const B: i32 = 100;


fn random_point(a: i32, b: i32) -> Point {
    let mut rng = rand::thread_rng();
    return (rng.gen_range(a..=b), rng.gen_range(a..=b))
}

fn benchmark_divide_and_conquer() {
    let ns = vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000];
    //let ns = vec![1000, 2000, 3000, 4000, 5000, 6000, 7000, 8000, 9000, 10000];

    for n in ns.clone() {
        let points: Vec<(i32, i32)> = (0..n).map(|_| random_point(A, B)).collect();
        let now = Instant::now();
        let dist = par_closest_distance(&points);
        let elapsed = now.elapsed().as_nanos();
        println!("{:.2?},", elapsed);
    }

    println!();

    for n in ns.clone() {
        let points: Vec<(i32, i32)> = (0..n).map(|_| random_point(A, B)).collect();
        let now = Instant::now();
        let dist = seq_closest_distance(&points);
        let elapsed = now.elapsed().as_nanos();
        println!("{:.2?},", elapsed);
    }

    println!();

    for n in ns {
        let points: Vec<(i32, i32)> = (0..n).map(|_| random_point(A, B)).collect();
        let now = Instant::now();
        let (p1, p2) = closest_pair(&points);
        let elapsed = now.elapsed().as_nanos();
        println!("{:.2?},", elapsed);
    }
}

fn main() {
    benchmark_divide_and_conquer();
}
