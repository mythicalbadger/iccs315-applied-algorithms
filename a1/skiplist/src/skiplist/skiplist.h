#include <iostream>
#include <cstdlib>

#define DEFAULT_P 0.5
#define DEFAULT_MAX_HEIGHT 1000

using namespace std;

class SkipListNode {
	public:
		SkipListNode(int key, int value, int level);
		int key;
		int value;
		SkipListNode ** forward;
};

class SkipList {
	private:
		double p;
		int maxLevel;
		int level;
		SkipListNode * header;
		double random();
		int randomLevel();
	public:
		SkipList(double p, int maxLevel);
		void insert(int key, int value);
		int get(int key);
		void remove(int key);	
};
