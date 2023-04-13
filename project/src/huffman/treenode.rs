/// A node in a Huffman tree
pub struct TreeNode {
    freq: i32,
    data: Option<char>,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>
}

impl TreeNode {
    /// Creates a new node
    pub fn new(data: Option<char>, freq: i32) -> Self {
        TreeNode { freq, data, left : None, right : None }
    }

    /// Assigns children to a node
    pub fn assign_children(&mut self, left: Option<Box<TreeNode>>, right: Option<Box<TreeNode>>) {
        self.left = left;
        self.right = right;
    }

    /// Checks to see if a node has data (is a leaf)
    pub fn has_data(&self) -> bool {
        self.data.is_some() 
    }

    /// Gets the data from a node
    pub fn get_data(&self) -> char {
        self.data.unwrap()
    }

    /// Checks to see if a node has a left child
    pub fn has_left(&self) -> bool {
        self.left.is_some()
    }

    /// Gets the left child of a node
    pub fn get_left(&self) -> &Box<TreeNode> {
        self.left.as_ref().unwrap()
    }

    /// Checks to see if a node has a right child
    pub fn has_right(&self) -> bool {
        self.right.is_some()
    }

    /// Gets the right child of a node
    pub fn get_right(&self) -> &Box<TreeNode> {
        self.right.as_ref().unwrap()
    }

    /// Gets the character frequency of a node
    pub fn get_freq(&self) -> i32 {
        self.freq
    }
}
