#include <iostream>
#include <algorithm>
#include <vector>
#include <math.h>
#include "hashedarraytree.h"

using namespace std;

/*
 * Hashed Array Tree constructor
 */
HashedArrayTree::HashedArrayTree() {
	length = 0;
	capacity = b*b;

	// Initialize data blocks
	data_blocks = new int * [b];
	for (int i = 0; i < b; i++) {
		data_blocks[i] = new int[b];
	}
}

/*
 * Returns Hashed Array Tree size
 */
int HashedArrayTree::size() {
	return length;
}

/*
 * Returns whether or not the Hashed Array Tree is full
 */
bool HashedArrayTree::isFull() {
	return length == capacity;
}

/*
 * Grows the Hashed Array Tree
 */
void HashedArrayTree::grow() {
	int newB = b*2;
	int k = log2(newB);
	int old_k = log2(b);

	// Initialize new data blocks
	int ** new_data_blocks = new int * [newB];
	for (int i = 0; i < newB; i++) {
		new_data_blocks[i] = new int [newB];
	}

	// Copy old stuff over
	for (int i = 0; i < length; i++) {
		new_data_blocks[i >> k][i & (newB - 1)] = data_blocks[i >> old_k][i & (b - 1)];
	}

	// Delete old data blocks
	for (int i = 0; i < b; i++) {
		delete[] data_blocks[i];
	}
	delete[] data_blocks;

	// Replace
	data_blocks = new_data_blocks;
	for (int i = b; i < newB; i++) {
		data_blocks[i] = new_data_blocks[i];
	}

	// Update b and capacity
	b = newB;
	capacity = b*b;
}

/**
 * Pushes an element to the end of Hashed Array Tree 
 */
void HashedArrayTree::push(int elt) {
	if (isFull()) {
		grow();
	}

	// Calculate index using bit shifting and insert
	int k = log2(b);
	int block_idx = length >> k;
	int idx = length & (b - 1);
	data_blocks[block_idx][idx] = elt;

	length++;
}

/*
 * Gets and element and index idx from Hashed Array Tree
 */
int HashedArrayTree::get(int idx) {
	// A bit different from indexing in class, which did not seem to work
	return data_blocks[idx >> (int) log2(b)][idx & (b - 1)];
}

/*
 * Returns string representation of the Hashed Array Tree
 */
string HashedArrayTree::toString() {
	string ret;
	int k = log2(b);
	for (int i = 0; i < length; i++) {
		int block_idx = i >> k;
		int idx = i & (b - 1);
		ret += to_string(data_blocks[block_idx][idx]) + ' ';
	}
	return ret;
}
