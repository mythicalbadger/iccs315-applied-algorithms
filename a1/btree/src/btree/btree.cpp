#include <iostream>
#include "btree.h"

using namespace std;

/*
 * If this code looks paraphrased, that's because *it was*
 *
 * References
 * - https://www.programiz.com/dsa/insertion-into-a-b-tree
 * - https://www.geeksforgeeks.org/insert-operation-in-b-tree/
 */

BTree::BTree(int a) {
	this->root = NULL;
	this->a = a;
}

BTree::~BTree() {
	root->deleteAll();
}

BTreeNode * BTree::search(int key) {
	if (root == NULL) 
		return root;
	else 
		return root->search(key);

}

void BTree::insert(int key) {
	if (!root) {
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
			newRoot->splitChild(0, root);

			int idx = newRoot->keys[0] < key ? 1 : 0;

			newRoot->children[idx]->insertKey(key);

			root = newRoot;
		}
	}
}
