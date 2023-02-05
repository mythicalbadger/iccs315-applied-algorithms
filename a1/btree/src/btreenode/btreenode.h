class BTreeNode {
	int a;
	int b;
	bool leaf;
	public:
		int size;
		int * keys;
		BTreeNode ** children;
		BTreeNode(int a, bool leaf);
		BTreeNode * search(int key);
		void insertKey(int k);
		void splitChild(int idx, BTreeNode * child);
		void setKey(int idx, int key);
		void displayKeys();
		bool isFull();
		void deleteAll();
};
