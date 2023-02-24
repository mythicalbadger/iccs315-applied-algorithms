use std::collections::HashSet;
use std::iter::FromIterator;
use std::cmp::Ordering;
use ordered_float::OrderedFloat;
use rand::Rng;
use core::arch::x86_64::_rdtsc;

#[derive(Copy, Clone, Hash)]
struct MixedRecord {
    i32_var: i32, 
    f32_var: OrderedFloat<f32>, 
    i64_var: i64, 
    f64_var: OrderedFloat<f64>
}

impl PartialEq for MixedRecord {
    fn eq(&self, other: &Self) -> bool {
        self.i32_var == other.i32_var && 
        self.f32_var == other.f32_var &&
        self.i64_var == other.i64_var &&
        self.f64_var == other.f64_var
    }
}

impl PartialOrd for MixedRecord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.i32_var.cmp(&other.i32_var).then(
        self.f32_var.cmp(&other.f32_var)).then(
        self.i64_var.cmp(&other.i64_var)).then(
        self.f64_var.cmp(&other.f64_var)))
    }
}

impl Ord for MixedRecord {
    fn cmp(&self, other: &Self) -> Ordering {
        self.i32_var.cmp(&other.i32_var).then(
        self.f32_var.cmp(&other.f32_var)).then(
        self.i64_var.cmp(&other.i64_var)).then(
        self.f64_var.cmp(&other.f64_var))
    }
}

impl Eq for MixedRecord {}


fn unique_hashset<T>(arr: &Vec<T>) -> Vec<T> 
where T: Ord + std::hash::Hash + Copy {
    let mut ret: Vec<T> = vec![];
    let mut unique_set: HashSet<T> = HashSet::new();
    let arr_copy = arr.clone();

    for val in arr_copy {
        unique_set.insert(val);
    }

    for val in unique_set {
        ret.push(val);
    }

    ret
}

fn unique_sorted<T>(arr: &Vec<T>) -> Vec<T> where T: Ord + Copy {
    let mut ret: Vec<T> = vec![];
    let mut arr_copy = arr.clone(); 
    arr_copy.sort();

    ret.push(arr[0]);

    for val in arr_copy {
        if ret[ret.len() - 1] != val {
            ret.push(val);
        }
    }

    ret
}

fn benchmarki32(n: i32, dedup_type: i32) {
    let mut input: Vec<i32> = Vec::with_capacity(n as usize);

    for _ in 0..n {
        input.push(rand::thread_rng().gen_range(0..=1000));
    }
    
    if dedup_type == 0 {
        let start: u64 = unsafe { _rdtsc() };
        unique_hashset(&input);
        let end: u64 = unsafe { _rdtsc() };

        print!("{:?}, ", end-start);
    }
    else {
        let start: u64 = unsafe { _rdtsc() };
        unique_sorted(&input);
        let end: u64 = unsafe { _rdtsc() };

        print!("{:?}, ", end-start);
    }

}

fn benchmarki64(n: i32, dedup_type: i32) {
    let mut input: Vec<i64> = Vec::with_capacity(n as usize);

    for _ in 0..n {
        input.push(rand::thread_rng().gen_range(0..=1000));
    }
    
    if dedup_type == 0 {
        let start: u64 = unsafe { _rdtsc() };
        unique_hashset(&input);
        let end: u64 = unsafe { _rdtsc() };

        print!("{:?}, ", end-start);
    }
    else {
        let start: u64 = unsafe { _rdtsc() };
        unique_sorted(&input);
        let end: u64 = unsafe { _rdtsc() };

        print!("{:?}, ", end-start);
    }
}

fn benchmarkf32(n: i32, dedup_type: i32) {
    let mut input: Vec<OrderedFloat<f32>> = Vec::with_capacity(n as usize);

    for _ in 0..n {
        input.push(OrderedFloat::from(rand::thread_rng().gen_range(0..=1000) as f32));
    }
    
    if dedup_type == 0 {
        let start: u64 = unsafe { _rdtsc() };
        unique_hashset(&input);
        let end: u64 = unsafe { _rdtsc() };

        print!("{:?}, ", end-start);
    }
    else {
        let start: u64 = unsafe { _rdtsc() };
        unique_sorted(&input);
        let end: u64 = unsafe { _rdtsc() };

        print!("{:?}, ", end-start);
    }
}

fn benchmarkf64(n: i32, dedup_type: i32) {
    let mut input: Vec<OrderedFloat<f64>> = Vec::with_capacity(n as usize);

    for _ in 0..n {
        input.push(OrderedFloat::from(rand::thread_rng().gen_range(0..=1000) as f64));
    }
    
    if dedup_type == 0 {
        let start: u64 = unsafe { _rdtsc() };
        unique_hashset(&input);
        let end: u64 = unsafe { _rdtsc() };

        print!("{:?}, ", end-start);
    }
    else {
        let start: u64 = unsafe { _rdtsc() };
        unique_sorted(&input);
        let end: u64 = unsafe { _rdtsc() };

        print!("{:?}, ", end-start);
    }
}

fn benchmark_string_short(n: i32, dedup_type: i32) {
    let charset = "abcdefghijklmnopqrstuvwxyz";
    let mut input: Vec<&str> = Vec::with_capacity(n as usize);
    let rand_str: Vec<String> = (0..n).map(|_| random_string::generate(5, charset)).collect();

    for i in 0..n {
        input.push(&rand_str[i as usize]);
    }

    if dedup_type == 0 {
        let start: u64 = unsafe { _rdtsc() };
        unique_hashset(&input);
        let end: u64 = unsafe { _rdtsc() };

        print!("{:?}, ", end-start);
    }
    else {
        let start: u64 = unsafe { _rdtsc() };
        unique_sorted(&input);
        let end: u64 = unsafe { _rdtsc() };

        print!("{:?}, ", end-start);
    }

}

fn benchmark_string_long(n: i32, dedup_type: i32) {
    let charset = "abcdefghijklmnopqrstuvwxyz";
    let mut input: Vec<&str> = Vec::with_capacity(n as usize);
    let rand_str: Vec<String> = (0..n).map(|_| random_string::generate(50, charset)).collect();

    for i in 0..n {
        input.push(&rand_str[i as usize]);
    }

    if dedup_type == 0 {
        let start: u64 = unsafe { _rdtsc() };
        unique_hashset(&input);
        let end: u64 = unsafe { _rdtsc() };

        print!("{:?}, ", end-start);
    }
    else {
        let start: u64 = unsafe { _rdtsc() };
        unique_sorted(&input);
        let end: u64 = unsafe { _rdtsc() };

        print!("{:?}, ", end-start);
    }
}

fn benchmark_mixed_record(n: i32, dedup_type: i32) {
    let charset = "abcdefghijklmnopqrstuvwxyz";
    let mut input: Vec<MixedRecord> = Vec::with_capacity(n as usize);

    for _ in 0..n {
        let i32_var: i32 = rand::thread_rng().gen_range(0..=1000);
        let i64_var: i64 = rand::thread_rng().gen_range(0..=1000);
        let f32_var: OrderedFloat<f32> = OrderedFloat::from(rand::thread_rng().gen_range(0i32..=1000) as f32);
        let f64_var: OrderedFloat<f64> = OrderedFloat::from(rand::thread_rng().gen_range(0i32..=1000) as f64);
        input.push(MixedRecord { i32_var, f32_var, i64_var, f64_var })
    }

    if dedup_type == 0 {
        let start: u64 = unsafe { _rdtsc() };
        unique_hashset(&input);
        let end: u64 = unsafe { _rdtsc() };

        print!("{:?}, ", end-start);
    }
    else {
        let start: u64 = unsafe { _rdtsc() };
        unique_sorted(&input);
        let end: u64 = unsafe { _rdtsc() };

        print!("{:?}, ", end-start);
    }

}

fn main() {
    println!("Benchmark i32 (HashSet)");
    for n in vec![10, 100, 1000, 10000, 100000, 1000000] {
        benchmarki32(n, 0);
    }
    println!();
    println!("Benchmark i32 (Sorted)");
    for n in vec![10, 100, 1000, 10000, 100000, 1000000] {
        benchmarki32(n, 1);
    }
    println!();
    println!("Benchmark i64 (HashSet)");
    for n in vec![10, 100, 1000, 10000, 100000, 1000000] {
        benchmarki64(n, 0);
    }
    println!();
    println!("Benchmark i64 (Sorted)");
    for n in vec![10, 100, 1000, 10000, 100000, 1000000] {
        benchmarki64(n, 1);
    }
    println!();

    println!("Benchmark f32 (HashSet)");
    for n in vec![10, 100, 1000, 10000, 100000, 1000000] {
        benchmarkf32(n, 0);
    }
    println!();
    println!("Benchmark f32 (Sorted)");
    for n in vec![10, 100, 1000, 10000, 100000, 1000000] {
        benchmarkf32(n, 1);
    }
    println!();
    println!("Benchmark f64 (HashSet)");
    for n in vec![10, 100, 1000, 10000, 100000, 1000000] {
        benchmarkf64(n, 0);
    }
    println!();
    println!("Benchmark f64 (Sorted)");
    for n in vec![10, 100, 1000, 10000, 100000, 1000000] {
        benchmarkf64(n, 1);
    }
    println!();

    println!("Benchmark String Short (HashSet)");
    for n in vec![10, 100, 1000, 10000, 100000, 1000000] {
        benchmark_string_short(n, 0);
    }
    println!();
    println!("Benchmark String Short (Sorted)");
    for n in vec![10, 100, 1000, 10000, 100000, 1000000] {
        benchmark_string_short(n, 1);
    }
    println!();

    println!("Benchmark String Long (HashSet)");
    for n in vec![10, 100, 1000, 10000, 100000, 1000000] {
        benchmark_string_long(n, 0);
    }
    println!();
    println!("Benchmark String Long (Sorted)");
    for n in vec![10, 100, 1000, 10000, 100000, 1000000] {
        benchmark_string_long(n, 1);
    }
    println!();
    println!("Mixed Record (HashSet)");
    for n in vec![10, 100, 1000, 10000, 100000, 1000000] {
        benchmark_mixed_record(n, 0);
    }
    println!();
    println!("Mixed Record (Sorted)");
    for n in vec![10, 100, 1000, 10000, 100000, 1000000] {
        benchmark_mixed_record(n, 1);
    }
    println!();
}

