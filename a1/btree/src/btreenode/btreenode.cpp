#include <iostream>
#include "btreenode.h"

/*
 * Constructor for the BTreeNode
 */
BTreeNode::BTreeNode(int a, bool leaf) {
	this->a = a;	
	this->b = 2*a;
	this->leaf = leaf;
	this->size = 0;
	this->keys = new int[b - 1];
	children = new BTreeNode * [b];
}

/*
 * Kind of failed semi-deconstructor for B-Tree node?
 */
void BTreeNode::deleteAll() {
    int idx;

	// Scan through the tree, recursively deleting all nodes
    for (idx = 0; idx < size; idx++) {
        if (!leaf)
            children[idx]->deleteAll();
    }
 
    // Print the subtree rooted with last child
    if (!leaf)
        children[idx]->deleteAll();

	delete this->keys;
	delete this->children;
	delete this;
}

/*
 * Displays the keys in the BTreeNode
 */
void BTreeNode::displayKeys() {
	for (int i = 0; i < size; i++) {
		std::cout << keys[i] << ' ';
	}
	std::cout << std::endl;
}

/*
 * Returns whether or not the BTreeNode is full
 */
bool BTreeNode::isFull() {
	return size == b - 1;
}

/*
 * Inserts a new key into the B-Tree node
 */
void BTreeNode::insertKey(int k) {
	int idx = size - 1;

	// If we have a leaf, need to find spot to insert key and shift keys after
	if (leaf) {
		while (idx >= 0 && keys[idx] > k) {
			keys[idx + 1] = keys[idx];
			idx--;
		}
		keys[idx + 1] = k;
		size++;
	}
	else {
		// Find child where new key can be inserted
		while (idx >= 0 && keys[idx] > k) {
			idx--;
		}

		// If child is full, we split
		if (children[idx + 1]->isFull()) {
			splitChild(idx + 1, children[idx + 1]);

			// After split, middle key is promoted and children[i] split into two
			idx = keys[idx + 1] < k ? idx : idx + 1;
		}

		children[idx + 1]->insertKey(k);
	}
}

/*
 * Splits a child of a node
 */
void BTreeNode::splitChild(int idx, BTreeNode * child) {
	// Create new node to store remainder
	BTreeNode * newNode = new BTreeNode(child->a, child->leaf);
	newNode->size = a - 1;

	// Copy last a-1 keys of child to new node
	for (int i = 0; i < a - 1; i++)
		newNode->keys[i] = child->keys[i + a];

	// Copy last a children of child to new node
	if (!child->leaf) {
		for (int i = 0; i < a; i++) {
			newNode->children[i] = child->children[i + a];
		}
	}

	// Reduce number of keys in child
	child->size = a - 1;

	// Create space for new child
	for (int i = size; i >= idx + 1; i--) {
		children[i + 1] = children[i];
	}

	// Link new child to current node
	children[idx + 1] = newNode;

	// Prepare for entry of new key of child moving into our node
	for (int i = size - 1; i >= idx; i--) {
		keys[i + 1] = keys[i];
	}

	// Copy middle key of child to current node and increment code
	keys[idx] = child->keys[a - 1];
	size++;
}

/**
 * Searches the BTreeNode (and children) for a key
 */
BTreeNode * BTreeNode::search(int key) {
	int idx;

	for (idx = 0; idx < size && key > keys[idx]; idx++)
		;

	if (keys[idx] == key)
		return this;
	else if (leaf)
		return NULL;
	else
		return children[idx]->search(key);
}
