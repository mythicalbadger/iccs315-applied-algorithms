#include <iostream>
#include <stdexcept>
#include <stdlib.h>
#include <bits/stdc++.h>
#include <ctime>
#include "skiplist_bidirectional.h"

SkipListNode::SkipListNode(int key, int value, int level) {
	this->key = key;
	this->value = value;
	this->forward = new SkipListNode * [level+1];
	this->level = level;
}

SkipList::SkipList(double p, int maxLevel) {
	this->p = p;
	this->level = 0;
	this->maxLevel = maxLevel;
	this->header = new SkipListNode(INT_MAX, INT_MAX, maxLevel);

	// Now we store pointer to the smallest node as well
	this->smallest = header;
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
	// Instead of starting at header, start at smallest
	SkipListNode * x = smallest;

	// If lucky, return
	if (x->key == key) return x->value;

	// Otherwise, start at the max level of the smallest node and go from there
	for (int i = smallest->level; i >= 0; i--) {
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

		if (newNode->key < smallest->key) {
			smallest = newNode;
		}
	}
}

void SkipList::remove(int key) {
	SkipListNode * update[maxLevel];
	SkipListNode * x = header;
	
	for (int i = level; i >= 0; i--) {
		while (x->forward[i] != NULL && x->forward[i]->key < key) {
			x = x->forward[i];
		}
		update[i] = x;
	}

	x = x->forward[0];

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
