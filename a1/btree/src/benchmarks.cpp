#include <iostream>
#include <cstdlib>
#include <bits/stdc++.h>
#include "btree/btree.h"

using namespace std;

static __inline__ unsigned long long rdtsc(void){    
    unsigned hi, lo;    
    __asm__ __volatile__ ("rdtsc" : "=a"(lo), "=d"(hi));    
    return ( (unsigned long long)lo)|( ((unsigned long long)hi)<<32   );    
}

void insertBenchmark() {
	int minB = -1;
	unsigned long long bestTime = INT_MAX;
	int bValues[] = { 10, 50, 100, 1000, 1500, 10000, 15000, 100000, 150000, 1000000, 1500000, 10000000 };
	unsigned long long bTimes[12];

	for (int i = 0; i < 12; i++) {
		BTree * btree = new BTree(bValues[i]);

		unsigned long long avg = 0;

		for (int j = 1; j < 10000000; j += 100) {
			unsigned long long start = rdtsc();
			btree->insert(rand() % 1000);
			unsigned long long end = rdtsc();
			unsigned long long time = end - start;
			avg += time;
		}

		bTimes[i] = avg / 10000000;
		cout << "Finished insertion for b: " << bValues[i] << endl;

		delete btree;
	}

	for (int i = 0;  i < 12; i++) {
		cout << bTimes[i] << ", ";
	}

}

void searchBenchmark() {
	int minB = -1;
	unsigned long long bestTime = INT_MAX;
	int bValues[] = { 10, 50, 100, 1000, 1500, 10000, 15000, 100000, 150000, 1000000, 1500000, 10000000 };
	unsigned long long bTimes[12];

	for (int i = 0; i < 12; i++) {
		BTree * btree = new BTree(bValues[i]);

		for (int j = 1; j < 10000000; j++) {
			btree->insert(j);
		}

		unsigned long long start = rdtsc();
		for (int j = 1; j < 10000000; j+= 1000) {
			btree->search(j);
		}
		unsigned long long end = rdtsc();
		unsigned long long time = end - start;

		bTimes[i] = time;

		delete btree;

		cout << "Finished searching for b: " << bValues[i] << endl;
	}

	for (int i = 0;  i < 12; i++) {
		cout << bTimes[i] << ", ";
	}

}
int main() {
	//insertBenchmark();
	searchBenchmark();
	
}
