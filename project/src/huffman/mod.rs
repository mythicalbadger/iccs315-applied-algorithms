pub mod treenode;
use treenode::TreeNode;
use std::collections::HashMap;

pub type Tree = Vec<Box<TreeNode>>;

/// Constructs the Huffman tree
pub fn construct_tree(data: &[u8]) -> Box<TreeNode> {
    // Generate character frequencies
    let freq: HashMap<char, i32> = char_freq(data);

    // Create an array of each node
    let mut tree: Tree = freq.iter().map(|ch| {
        Box::new(TreeNode::new(Some(*ch.0), *ch.1))
    }).collect();

    // Sort trees according to pseudocode in writeup
    while tree.len() > 1 {
        tree.sort_by(|x, y| y.get_freq().cmp(&x.get_freq()));

        // Here x is the node with smaller frequency and y is the larger
        let x = tree.pop().unwrap();
        let y = tree.pop().unwrap();

        // Create the 'parent' node
        let mut node = Box::new(TreeNode::new(None, x.get_freq() + y.get_freq()));
        node.assign_children(Some(x), Some(y));

        // Push our new node back
        tree.push(node);
    }

    // Root is the only remaining item in array
    let root = tree.pop().unwrap();

    root
}

/// Generates a HashMap/histogram of character frequencies
pub fn char_freq(data: &[u8]) -> HashMap<char, i32> {
    let mut freq: HashMap<char, i32> = HashMap::new();
    data.iter().for_each(|ch| {
        *freq.entry(*ch as char).or_insert(0) += 1
    });
    freq
}

/// Assigns Huffman codes 
pub fn assign_codes(node: &Box<TreeNode>, dict: &mut HashMap<char, String>, st: String) {
    if node.has_data() { dict.insert(node.get_data(), st); }

    else {
        if node.has_left() { assign_codes(node.get_left(), dict, st.clone() + "0") }
        if node.has_right() { assign_codes(node.get_right(), dict, st.clone() + "1") }
    }
}

/// Huffman compression
pub fn compress(data: &[u8], dict: &HashMap<char, String>) -> String {
    let mut out = String::new();

    data.iter().for_each(|&ch| {
        out.push_str(dict.get(&(ch as char)).unwrap())
    });

    out
}

/// Huffman decompression
pub fn decompress(encoded: &str, root: &Box<TreeNode>) -> Vec<u8> {
    let mut out = vec![];
    let mut curr = root;

    encoded.chars().for_each(|ch| {
        if ch == '0' {
            if curr.has_left() { curr = curr.get_left(); }
        }
        else {
            if curr.has_right() { curr = curr.get_right(); }
        }

        if curr.has_data() { out.push(curr.get_data() as u8); curr = root; }
    });

    out
}

/// Calculates the required space for a given Huffman encoding
pub fn calculate_size(dict: &mut HashMap<char, String>, freq: HashMap<char, i32>) -> f32 {
    // Number of bits used
    let bits: i32 = dict.iter().map(|(k, v)| freq[k] * v.len() as i32).sum();
    bits as f32 / 8.0
}
