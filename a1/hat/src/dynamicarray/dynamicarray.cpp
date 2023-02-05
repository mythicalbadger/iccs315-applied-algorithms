#include <iostream>
#include "dynamicarray.h"

using namespace std;

DynamicArray::DynamicArray() {
	length = 0;
	capacity = DEFAULT_ARRAY_SIZE;
	frontIdx = 0;
	tailIdx = 0;
	data = new int [capacity];
}
int DynamicArray::size() {
	return length;
}

bool DynamicArray::isFull() {
	return length == capacity;
}

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

void DynamicArray::push(int elt) {
	if (isFull()) {
		grow();
	}

	data[tailIdx++] = elt;
	length++;
}

int DynamicArray::get(int idx) {
	// I trust the user completely
	return data[idx];
}

string DynamicArray::toString() {
	string ret;

	for (int i = frontIdx; i < frontIdx + length; i++) {
		ret += to_string(data[i]);
	}

	return ret;
}
