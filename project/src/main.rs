mod lzw;
mod rle;
mod huffman;
mod benchmarks;
mod file_reader;

fn main() {
    // benchmarks::benchmark_sentence();
    // benchmarks::benchmark_hhgttg();
    // benchmarks::benchmark_complex();
    benchmarks::benchmark_unique();
}
