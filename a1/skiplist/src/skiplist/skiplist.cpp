#include <iostream>
#include <stdexcept>
#include <stdlib.h>
#include <ctime>
#include "skiplist.h"

/*
 * === Implementation References ===
 * - Pugh, W. (1990). Skip lists: a probabilistic alternative to balanced trees. Communications of the ACM, 33(6), 668-676.
 * - https://www.geeksforgeeks.org/skip-list-set-2-insertion/
 */ 

SkipListNode::SkipListNode(int key, int value, int level) {
	this->key = key;
	this->value = value;
	this->forward = new SkipListNode * [level+1];
}

SkipList::SkipList(double p, int maxLevel) {
	this->p = p;
	this->level = 0;
	this->maxLevel = maxLevel;

	// Header node to be sentinel
	this->header = new SkipListNode(-1, -1, maxLevel);

	// In order for more random random
	srand((unsigned int)time(NULL));
}

double SkipList::random() {
	double rng = (double) rand() / RAND_MAX;
	return rng;
}

int SkipList::randomLevel() {
	int level = 0;
	
	while (random() < p && level < maxLevel)
		level++;

	return level;
}

int SkipList::get(int key) {
	SkipListNode * x = header;

	// Start at top level and search
	for (int i = level; i >= 0; i--) {
		while (x->forward[i] != NULL && x->forward[i]->key < key) {
			x = x->forward[i];
		}
	}

	x = x->forward[0];

	if (x != NULL && x->key == key)
		return x->value;
	throw out_of_range("Key not in list"); // out of range kinda not exact but T_T
}

void SkipList::insert(int key, int value) {
	// update[i] contains pointer to rightmost node of level i
	SkipListNode * update[maxLevel + 1];

	// Pointer to our header node
	SkipListNode * x = header;

	// Search each level for a suitable place to insert
	for (int i = level; i >= 0; i--) {
		while (x->forward[i] != NULL && x->forward[i]->key < key) {
			x = x->forward[i];
		}
		update[i] = x;
	}

	x = x->forward[0];

	// If place to insert doesn't have our key already, insert and update pointers
	if (x == NULL || x->key != key) {
		int nodeLevel = randomLevel();
		if (nodeLevel > level) {
			for (int i = level + 1; i <= nodeLevel; i++) {
				update[i] = header;
			}
			level = nodeLevel;
		}
		SkipListNode * newNode = new SkipListNode(key, value, nodeLevel);

		for (int i = 0; i <= nodeLevel; i++) {
			newNode->forward[i] = update[i]->forward[i];
			update[i]->forward[i] = newNode;
		}
	}
}

void SkipList::remove(int key) {
	SkipListNode * update[maxLevel];
	SkipListNode * x = header;
	
	// Find element
	for (int i = level; i >= 0; i--) {
		while (x->forward[i] != NULL && x->forward[i]->key < key) {
			x = x->forward[i];
		}
		update[i] = x;
	}

	x = x->forward[0];

	// Once found, update pointers and delete it
	if (x != NULL && x->key == key) {
		for (int i = 0; i <= level; i++) {
			if (update[i]->forward[i] != x) break;
			update[i]->forward[i] = x->forward[i];
		}
		delete x;

		while (level >= 0 && header->forward[level] != NULL) {
			level--;	
		}
	}
}
