#include <iostream>

/**
 * (Maybe non)Standard Hashed Array Tree implementation
 */

#define DEFAULT_SIZE 2

class HashedArrayTree {
	private:
		int b = DEFAULT_SIZE;
		int length;
		int capacity;
		int ** data_blocks;
	public:
		HashedArrayTree();
		int size();
		bool isFull();
		void grow();
		void push(int elt);
		int get(int idx);
		std::string toString();
};
