#include <iostream>
#include "dynamicarray.h"

using namespace std;

/*
 * Constructor for Dynamic Array
 */
DynamicArray::DynamicArray() {
	length = 0;
	capacity = DEFAULT_ARRAY_SIZE;
	frontIdx = 0;
	tailIdx = 0;
	data = new int [capacity];
}

/*
 * Gets size of array
 */
int DynamicArray::size() {
	return length;
}

/*
 * Returns whether or not array is full
 */
bool DynamicArray::isFull() {
	return length == capacity;
}

/*
 * Doubles the array size
 */
void DynamicArray::grow() {
	int newCapacity = capacity * 2;
	int * new_data = new int [newCapacity];

	for (int i = 0; i < capacity; i++) {
		new_data[i] = data[i];
	}

	delete[] data;
	data = new_data;
	frontIdx = 0;
	tailIdx = length;
	capacity = newCapacity;
}

/*
 * Pushes an element elt into the end of the array
 */
void DynamicArray::push(int elt) {
	if (isFull()) {
		grow();
	}

	data[tailIdx++] = elt;
	length++;
}

/*
 * Gets an element at index idx in the array
 */
int DynamicArray::get(int idx) {
	// I trust the user completely
	return data[idx];
}

/*
 * Returns string representation of the array
 */
string DynamicArray::toString() {
	string ret;

	for (int i = frontIdx; i < frontIdx + length; i++) {
		ret += to_string(data[i]);
	}

	return ret;
}
