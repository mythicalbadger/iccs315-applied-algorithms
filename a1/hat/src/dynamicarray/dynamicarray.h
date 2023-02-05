#include <iostream>

/*
 * Standard geometric expansion dynamic array (with less bells and whistles)
 *
 */

#define DEFAULT_ARRAY_SIZE 8

class DynamicArray {
	private:
		int length;
		int capacity;
		int frontIdx;
		int tailIdx;
		int * data;
	public:
		DynamicArray();
		int size();
		bool isFull();
		void grow();
		void push(int elt);
		int get(int idx);
		std::string toString();
};
