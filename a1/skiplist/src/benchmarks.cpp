#include <iostream>
#include <bits/stdc++.h>
#include <stdlib.h>
#include "skiplist/skiplist.h"
#include <fstream>
#include <cmath>
#include <map>

using namespace std;

#define N 8192
#define ELT_RAND_MAX 16

static __inline__ unsigned long long rdtsc(void){
    unsigned hi, lo;  
    __asm__ __volatile__ ("rdtsc" : "=a"(lo), "=d"(hi));
    return ( (unsigned long long)lo)|( ((unsigned long long)hi)<<32   );
}

void insertBenchmark() {
	for (int i = 0; i < 10; i++) {
		SkipList * skipList = new SkipList(DEFAULT_P, DEFAULT_MAX_HEIGHT);
		map<int, int> orderedMap;
		int keys[N];

		for (int j = 0; j < N; j++) {
			keys[j] = rand() % ELT_RAND_MAX;
		}

		cout << "=== INSERT TEST ===" << endl;
		unsigned long long cpu_start = rdtsc();
		for (int j = 0; j < N; j++) {
			orderedMap.insert({ keys[j], j });
		}
		unsigned long long cpu_finish = rdtsc();

		std::cout << "MAP: Total CPU Clocks:   " << cpu_finish - cpu_start << std::endl;

		cpu_start = rdtsc();
		for (int j = 0; j < N; j++) {
			skipList->insert(keys[j], j);
		}
		cpu_finish = rdtsc();

		std::cout << "HAT: Total CPU Clocks:   " << cpu_finish - cpu_start << std::endl;

		cout << endl;

		delete skipList;
	}
}

void averageInsertBenchmark() {
	SkipList * skipList = new SkipList(DEFAULT_P, DEFAULT_MAX_HEIGHT);
    map<int, int> orderedMap;
	int keys[N];

	for (int i = 0; i < N; i++) {
        keys[i] = rand() % ELT_RAND_MAX;
	}

	unsigned long long * skipListData = new unsigned long long[N];
	unsigned long long * orderedMapData = new unsigned long long[N];

	cout << "=== AVERAGE INSERT TEST ===" << endl;
	for (int i = 0; i < N; i++) {
		unsigned long long cpu_start = rdtsc();
		orderedMap.insert({ keys[i], i });
		unsigned long long cpu_finish = rdtsc();
		orderedMapData[i] = cpu_finish - cpu_start;
	}

	for (int i = 0; i < N; i++) {
		unsigned long long cpu_start = rdtsc();
		skipList->insert(keys[i], i);
		unsigned long long cpu_finish = rdtsc();
		skipListData[i] = cpu_finish - cpu_start;
	}
	cout << endl;

	fstream skipListFile;
	fstream orderedMapFile;
	skipListFile.open("../data/skipListIndividualPushData.txt", ios_base::out);
	orderedMapFile.open("../data/orderedMapIndividualPushData.txt", ios_base::out);

	for (int i = 0; i < N; i++) {
		skipListFile << skipListData[i] << ',';
		orderedMapFile << orderedMapData[i] << ',';
	}

	skipListFile.close();
	orderedMapFile.close();

	delete skipList;

	cout << endl;
}

void averageGetBenchmark() {
	SkipList * skipList = new SkipList(DEFAULT_P, DEFAULT_MAX_HEIGHT);
	map<int, int> orderedMap;
	int keys[N];

	for (int i = 0; i < N; i++) {
		keys[i] = rand() % ELT_RAND_MAX;
	}

	for (int i = 0; i < N; i++) {
		orderedMap.insert({ keys[i], i });
		skipList->insert(keys[i], i);
	}

	cout << "=== GET TEST ===" << endl;
	unsigned long long * skipListData = new unsigned long long[N];
	unsigned long long * orderedMapData = new unsigned long long[N];

	for (int i = 0; i < N; i++) {
		unsigned long long cpu_start = rdtsc();
		auto temp = orderedMap.find(keys[i]);
		unsigned long long cpu_finish = rdtsc();
		orderedMapData[i] = cpu_finish - cpu_start;
	}

	for (int i = 0; i < N; i++) {
		unsigned long long cpu_start = rdtsc();
		skipList->get(keys[i]);
		unsigned long long cpu_finish = rdtsc();
		skipListData[i] = cpu_finish - cpu_start;
	}
	cout << endl;

	fstream skipListFile;
	fstream orderedMapFile;
	skipListFile.open("../data/skipListIndividualGetData.txt", ios_base::out);
	orderedMapFile.open("../data/orderedMapIndividualGetData.txt", ios_base::out);

	for (int i = 0; i < N; i++) {
		skipListFile << skipListData[i] << ',';
		orderedMapFile << orderedMapData[i] << ',';
	}

	skipListFile.close();
	orderedMapFile.close();

	delete skipList;

	cout << endl;
}

void removeBenchmark() {
	SkipList * skipList = new SkipList(DEFAULT_P, DEFAULT_MAX_HEIGHT);
	map<int, int> orderedMap;
	int keys[N];

	for (int i = 0; i < N; i++) {
		keys[i] = rand() % ELT_RAND_MAX;
	}

	for (int i = 0; i < N; i++) {
		orderedMap.insert({ keys[i], i });
		skipList->insert(keys[i], i);
	}

	cout << "=== REMOVE TEST ===" << endl;
	unsigned long long * skipListData = new unsigned long long[N];
	unsigned long long * orderedMapData = new unsigned long long[N];

	for (int i = 0; i < N; i++) {
		unsigned long long cpu_start = rdtsc();
		orderedMap.erase(keys[i]);
		unsigned long long cpu_finish = rdtsc();
		orderedMapData[i] = cpu_finish - cpu_start;
	}

	for (int i = 0; i < N; i++) {
		unsigned long long cpu_start = rdtsc();
		skipList->remove(keys[i]);
		unsigned long long cpu_finish = rdtsc();
		skipListData[i] = cpu_finish - cpu_start;
	}
	cout << endl;

	fstream skipListFile;
	fstream orderedMapFile;
	skipListFile.open("skipListIndividualRemoveData.txt", ios_base::out);
	orderedMapFile.open("orderedMapIndividualRemoveData.txt", ios_base::out);

	for (int i = 0; i < N; i++) {
		skipListFile << skipListData[i] << ',';
		orderedMapFile << orderedMapData[i] << ',';
	}

	skipListFile.close();
	orderedMapFile.close();

	delete skipList;

	cout << endl;

}

void bestPInsertBenchmark() {
	double bestP = -1;
	unsigned long long bestTime = LONG_MAX;
	double pValues[] = { 0.5, 1/exp(1.0), 0.25, 0.125, 0.0625 };
	unsigned long long bTimes[5];
	int keys[8192];

	for (int i = 0; i < 8192; i++) {
		keys[i] = rand() % ELT_RAND_MAX;
	}

	for (int i = 0; i < 5; i++) {
        SkipList * skipList = new SkipList(pValues[i], DEFAULT_MAX_HEIGHT);
		unsigned long long avg = 0;

		for (int j = 0; j < 8192; j++) {
			unsigned long long start = rdtsc();
			skipList->insert(keys[j], j);
			unsigned long long end = rdtsc();
			unsigned long long time = end - start;
			avg += time;
		}

		bTimes[i] = avg / 8192;
		cout << "Finished insertion for p: " << pValues[i] << endl;

		delete skipList;
	}

	for (int i = 0;  i < 5; i++) {
		cout << bTimes[i] << ", ";
	}

}

void bestPSearchBenchmark() {
	double bestP = -1;
	unsigned long long bestTime = LONG_MAX;
	double pValues[] = { 0.5, 1/exp(1.0), 0.25, 0.125, 0.0625 };
	unsigned long long bTimes[5];
	int keys[8192];

	for (int i = 0; i < 8192; i++) {
		keys[i] = rand() % ELT_RAND_MAX;
	}


	for (int i = 0; i < 5; i++) {
        SkipList * skipList = new SkipList(pValues[i], DEFAULT_MAX_HEIGHT);
		for (int j = 0; j < 8192; j++) {
			skipList->insert(keys[j], j);
		}
		unsigned long long avg = 0;

		for (int j = 0; j < 8192; j++) {
			unsigned long long start = rdtsc();
			skipList->get(keys[j]);
			unsigned long long end = rdtsc();
			unsigned long long time = end - start;
			avg += time;
		}

		bTimes[i] = avg / 8192;
		cout << "Finished get for p: " << pValues[i] << endl;

		delete skipList;
	}

	for (int i = 0;  i < 5; i++) {
		cout << bTimes[i] << ", ";
	}

}


int main() {
	// Running all these at once is guaranteed segfault
	insertBenchmark();
	// averageInsertBenchmark();
	// averageGetBenchmark();
	// removeBenchmark();
	// bestPInsertBenchmark();
	// bestPSearchBenchmark();
}
