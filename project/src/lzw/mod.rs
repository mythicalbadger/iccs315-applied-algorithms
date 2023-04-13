use std::collections::HashMap;

const CODE: u32 = 255;

/// Compresses data using LZW encoding
pub fn compress(data: &[u8]) -> Vec<u32> {
    // Our encoding dictionary initialized with `CODE` values
    let mut dict: HashMap<Vec<u8>, u32> = (0u32..=CODE).map(|i| (vec![i as u8], i)).collect();
    let mut curr = vec![];
    let mut out = vec![];

    for &d in data {
        // Try extending our current phrase
        let mut curr_clone = curr.clone();
        curr_clone.push(d);

        if dict.contains_key(&curr_clone) {
            curr = curr_clone;
        }
        else {
            // If not in our dictionary, push encoding to output and update current phrase
            out.push(dict[&curr]);
            dict.insert(curr_clone, dict.len() as u32);
            curr.clear();
            curr.push(d);
        }
    }

    // Any leftover data?
    if !curr.is_empty() { out.push(dict[&curr]) }
    out
}

/// Decompresses data encoded with LZW encoding
pub fn decompress(data: &[u32]) -> Vec<u8> {
    // Decoding dictionary
    let mut dict: HashMap<u32, Vec<u8>> = (0u32..=CODE).map(|i| (i, vec![i as u8])).collect();
    let mut curr = dict[&data[0]].clone();
    let mut out = curr.clone();
    let mut entry;

    for d in &data[1..] {
        // If current phrase in dict, get it, otherwise extend
        if dict.contains_key(d) {
            entry = dict[d].clone();
        }
        else {
            entry = curr.clone();
            entry.push(curr[0]);
        }

        // Push to our output and update decoding dict
        out.extend_from_slice(&entry);
        curr.push(entry[0]);
        dict.insert(dict.len() as u32, curr);

        curr = entry;
    }

    out
}
