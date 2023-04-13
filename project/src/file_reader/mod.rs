use std::io::{ Read, BufReader};
use std::fs::File;

/// Reads in a file located in the /res/ folder.
///
/// Basic usage:
/// ```
/// let data: Vec<u8> = read_file("filename.txt").unwrap();
/// ```
pub fn read_file(filename: &str) -> Result<Vec<u8>, std::io::Error> {
    let filepath = env!("CARGO_MANIFEST_DIR").to_owned() + "/res/" + filename;
    let file = File::open(filepath)?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();

    reader.read_to_end(&mut buffer)?;

    Ok(buffer)
}
