#include <iostream>
#include "btree.h"

using namespace std;

/*
 * If this code looks paraphrased, that's because *it was*
 * Initial B-Tree implementation was bugged and had to redo benchmarks all over again so wanted them done right
 *
 * References
 * - https://www.programiz.com/dsa/insertion-into-a-b-tree
 * - https://www.geeksforgeeks.org/insert-operation-in-b-tree/
 */

/*
 * Constructor for the B-Tree
 */
BTree::BTree(int a) {
	this->root = NULL;
	this->a = a;
}

/**
 * Destructor for the B-Tree
 */
BTree::~BTree() {
	root->deleteAll();
}

/*
 * Searches the B-tree for a key
 */
BTreeNode * BTree::search(int key) {
	if (root == NULL) 
		return root;
	else 
		return root->search(key);

}

/*
 * Inserts a key into the B-tree
 */
void BTree::insert(int key) {
	if (!root) {
		// If no root, then we just create a new root (happy ending)
		root = new BTreeNode(a, true);
		root->keys[0] = key;
		root->size = 1;
	}
	else {
		// If root not full, yay
		if (!root->isFull()) {
			return root->insertKey(key);
		}
		else {
			// If root not full, oh noes
			BTreeNode * newRoot = new BTreeNode(a, false);
			newRoot->children[0] = root;

			// Split at root
			newRoot->splitChild(0, root);

			// Determine the side that the key should be on
			int idx = newRoot->keys[0] < key ? 1 : 0;

			// Insert the new key and update root
			newRoot->children[idx]->insertKey(key);

			root = newRoot;
		}
	}
}
