use std::collections::HashMap;
use std::fmt;
use std::time::Instant;
use crate::*;

struct BenchmarkData {
    compress_time: u128,
    decompress_time: u128,
    size_before: f32,
    size_after: f32,
    compression_ratio: f32
}

impl fmt::Display for BenchmarkData {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(&format!("Compression Time: {}\n", self.compress_time))?;
        fmt.write_str(&format!("Decompression Time: {}\n", self.decompress_time))?;
        fmt.write_str(&format!("Size Before: {}\n", self.size_before))?;
        fmt.write_str(&format!("Size After: {}\n", self.size_after))?;
        fmt.write_str(&format!("Compression Ratio: {}\n", self.compression_ratio))?;
        Ok(())
    }
}

fn benchmark_huffman(data: &[u8]) -> BenchmarkData {
    let (mut now, compress_time, decompress_time, compression_ratio);
    let mut root = huffman::construct_tree(&data);

    let mut dict: HashMap<char, String> = HashMap::new();
    let freq: HashMap<char, i32> = huffman::char_freq(&data);

    huffman::assign_codes(&root, &mut dict, "".to_string());

    now = Instant::now();
    let compressed = huffman::compress(&data, &dict);
    compress_time = now.elapsed().as_millis();

    now = Instant::now();
    let decompressed = huffman::decompress(&compressed, &root);
    decompress_time = now.elapsed().as_millis();


    let size_before = data.len() as f32;
    let size_after = huffman::calculate_size(&mut dict, freq) as f32;
    compression_ratio = size_before / size_after;

    BenchmarkData { compress_time, decompress_time, size_before, size_after, compression_ratio }
}

fn benchmark_lzw(data: &[u8]) -> BenchmarkData {
    let (mut now, compress_time, decompress_time, compression_ratio);

    now = Instant::now();
    let compressed = lzw::compress(&data);
    compress_time = now.elapsed().as_millis();

    now = Instant::now();
    let decompressed = lzw::decompress(&compressed);
    decompress_time = now.elapsed().as_millis();

    let size_before = data.len() as f32;
    let size_after = (compressed.len() * 4) as f32;
    compression_ratio = size_before / size_after;

    BenchmarkData { compress_time, decompress_time, size_before, size_after, compression_ratio }

}

fn benchmark_rle_normal(data: &[u8]) -> BenchmarkData {
    let (mut now, compress_time, decompress_time, compression_ratio);

    now = Instant::now();
    let compressed = rle::compress(&data);
    compress_time = now.elapsed().as_millis();

    now = Instant::now();
    let decompressed = rle::decompress(&compressed);
    decompress_time = now.elapsed().as_millis();

    let size_before = data.len() as f32;
    let size_after = compressed.len() as f32;
    compression_ratio = size_before / size_after;

    BenchmarkData { compress_time, decompress_time, size_before, size_after, compression_ratio }
}

fn benchmark_rle_write(data: &[u8]) -> BenchmarkData {
    let (mut now, compress_time, decompress_time, compression_ratio);

    now = Instant::now();
    let compressed = rle::compress_benchmark(&data);
    compress_time = now.elapsed().as_millis();

    now = Instant::now();
    let decompressed = rle::decompress_benchmark(&compressed);
    decompress_time = now.elapsed().as_millis();

    let size_before = data.len() as f32;
    let size_after = (std::fs::metadata(compressed).unwrap().len()) as f32;
    compression_ratio = size_before / size_after;

    BenchmarkData { compress_time, decompress_time, size_before, size_after, compression_ratio }
}

pub fn benchmark_sentence() {
    println!("=== SENTENCE BENCHMARK ===");
    let data = "The quick brown fox jumps over the lazy dog".as_bytes();
    
    let huffman = benchmark_huffman(&data);
    let lzw = benchmark_lzw(&data);
    let rle = benchmark_rle_normal(&data);

    println!("Huffman");
    println!("{}", huffman.to_string());

    println!("LZW");
    println!("{}", lzw.to_string());

    println!("RLE");
    println!("{}", rle.to_string());
}

pub fn benchmark_hhgttg() {
    println!("=== HHGTTG BENCHMARK ===");
    let data = file_reader::read_file("hhgttg.txt").unwrap();
    
    let huffman = benchmark_huffman(&data);
    let lzw = benchmark_lzw(&data);
    let rle = benchmark_rle_write(&data);

    println!("Huffman");
    println!("{}", huffman.to_string());

    println!("LZW");
    println!("{}", lzw.to_string());

    println!("RLE");
    println!("{}", rle.to_string());
}

pub fn benchmark_complex() {
    println!("=== COMPLEX BENCHMARK ===");
    let data = file_reader::read_file("complex.txt").unwrap();
    
    let huffman = benchmark_huffman(&data);
    let lzw = benchmark_lzw(&data);
    let rle = benchmark_rle_write(&data);

    println!("Huffman");
    println!("{}", huffman.to_string());

    println!("LZW");
    println!("{}", lzw.to_string());

    println!("RLE");
    println!("{}", rle.to_string());
}

pub fn benchmark_unique() {
    let data = file_reader::read_file("5mbzeros.txt").unwrap();
    
    let huffman = benchmark_huffman(&data);
    let lzw = benchmark_lzw(&data);
    let rle = benchmark_rle_normal(&data);

    println!("Huffman");
    println!("{}", huffman.to_string());

    println!("LZW");
    println!("{}", lzw.to_string());

    println!("RLE");
    println!("{}", rle.to_string());
}
