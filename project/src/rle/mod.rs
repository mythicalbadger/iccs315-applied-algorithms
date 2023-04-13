#![allow(dead_code)]

/// Compresses data using RLE encoding
pub fn compress(data: &[u8]) -> String {
    let n = data.len();
    let mut out = String::new();
    let mut count = 0;

    (0..n).for_each(|i| {
        count += 1;

        if i + 1 == n || data[i] != data[i+1] {
            if count > 0 { out.push_str(&count.to_string()); }
            out.push(data[i] as char);
            count = 0;
        }
    });

    out
}

/// Decompresses RLE encoded data
pub fn decompress(data: &str) -> Vec<u8> {
    let mut out: Vec<u8> = vec![];
    let mut numeric = String::new();

    data.chars().for_each(|ch| {
        if ch.is_numeric() { numeric.push(ch); }
        else {
            out.extend(ch.to_string().repeat(numeric.parse::<usize>().unwrap_or(1)).chars().map(|c| c as u8));
            numeric.clear();
        }
    });

    out
}

/// RLE compression for benchmarks. Writes to file instead of vector.
pub fn compress_benchmark(data: &[u8]) -> String {
    use std::fs::OpenOptions;
    use std::io::prelude::*;

    let filepath = env!("CARGO_MANIFEST_DIR").to_owned() + "/out/" + "rle.tmp";
    
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(filepath.clone())
        .unwrap();

    let n = data.len();
    let mut count = 0;

    (0..n).for_each(|i| {
        count += 1;

        if i + 1 == n || data[i] != data[i+1] {
            if count > 0 { 
                if let Err(e) = write!(file, "{:?}", &count.to_string()) {
                    eprintln!("Couldn't write to file: {}", e);
                }
            }
                if let Err(e) = write!(file, "{:?}", data[i] as char) {
                    eprintln!("Couldn't write to file: {}", e);
                }
            count = 0;
        }
    });

    filepath
}

/// RLE decompression for benchmark. Reads from file instead of input.
pub fn decompress_benchmark(filename: &str) -> Result<Vec<u8>, std::io::Error> {
    use std::io::{ Read, BufReader};
    use std::fs::File;

    let mut out: Vec<u8> = vec![];
    let mut numeric = String::new();
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();

    reader.read_to_end(&mut buffer)?;

    let data = buffer;

    data.iter().for_each(|&ch| {
        if (ch as char).is_numeric() { numeric.push(ch as char); }
        else {
            out.extend(ch.to_string().repeat(numeric.parse::<usize>().unwrap_or(1)).chars().map(|c| c as u8));
            numeric.clear();
        }
    });

    Ok(out)
}
