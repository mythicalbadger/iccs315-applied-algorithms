#include "../btreenode/btreenode.h"

class BTree {
	public:
		BTreeNode * root;
		int a;
		BTree(int a);
		~BTree();
		BTreeNode * search(int key);
		void insert(int key);
};
