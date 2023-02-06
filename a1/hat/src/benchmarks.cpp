#include <iostream>
#include <fstream>
#include "dynamicarray/dynamicarray.h"
#include "hashedarraytree/hashedarraytree.h"

using namespace std;

#define N 8192

/*
 * rdtsc implementation from Stack Overflow (sus?)
 */
static __inline__ unsigned long long rdtsc(void){
    unsigned hi, lo;  
    __asm__ __volatile__ ("rdtsc" : "=a"(lo), "=d"(hi));
    return ( (unsigned long long)lo)|( ((unsigned long long)hi)<<32   );
}

/*
 * Benchmarks overall throughput 
 */
void pushBenchmark() {
	HashedArrayTree * hat = new HashedArrayTree();
	DynamicArray * vec = new DynamicArray();

	cout << "=== PUSH BENCHMARK ===" << endl;

	// Test dynamic array
	unsigned long long cpu_start = rdtsc();
	for (int i = 0; i < N; i++) {
		vec->push(i);
	}
	unsigned long long cpu_finish = rdtsc();

    std::cout << "VEC: Total CPU Clocks:   " << cpu_finish - cpu_start << std::endl;

	// Test hashed array tree
	cpu_start = rdtsc();
	for (int i = 0; i < N; i++) {
		hat->push(i);
	}
	cpu_finish = rdtsc();

    std::cout << "HAT: Total CPU Clocks:   " << cpu_finish - cpu_start << std::endl;

	cout << endl;

	delete hat;
	delete vec;
}

/*
 * Benchmarks append latency
 */
void averagePushBenchmark() {
	HashedArrayTree * hat = new HashedArrayTree();
	DynamicArray * vec = new DynamicArray();

	unsigned long long * hatData = new unsigned long long[N];
	unsigned long long * vecData = new unsigned long long[N];

	cout << "=== AVERAGE PUSH TEST ===" << endl;
	for (int i = 0; i < N; i++) {
		unsigned long long cpu_start = rdtsc();
		vec->push(i);
		unsigned long long cpu_finish = rdtsc();
		vecData[i] = cpu_finish - cpu_start;
	}

	for (int i = 0; i < N; i++) {
		unsigned long long cpu_start = rdtsc();
		hat->push(i);
		unsigned long long cpu_finish = rdtsc();
		hatData[i] = cpu_finish - cpu_start;
	}
	cout << endl;

	fstream hatFile;
	fstream vecFile;
	hatFile.open("../data/hatAvgPushCycles.txt", ios_base::out);
	vecFile.open("../data/vecAvgPushCycles.txt", ios_base::out);

	for (int i = 0; i < N; i++) {
		hatFile << hatData[i] << ',';
		vecFile << vecData[i] << ',';
	}

	hatFile.close();
	vecFile.close();

	delete hat;
	delete vec;

	cout << endl;
}

/*
 * Benchmarks scan latency
 */
void accessBenchmark() {
	HashedArrayTree * hat = new HashedArrayTree();
	DynamicArray * vec = new DynamicArray();

	for (int i = 0; i < N; i++) {
		vec->push(i);
		hat->push(i);
	}

	cout << "=== SCAN TEST ===" << endl;

	unsigned long long cpu_start = rdtsc();
	for (int i = 0; i < N; i++) {
		int temp = vec->get(i);
	}
	unsigned long long cpu_finish = rdtsc();
	unsigned long long avg_cpu_clocks = (cpu_finish - cpu_start) / N;

    std::cout << "VEC: Avg CPU Clocks:   " << avg_cpu_clocks << std::endl;

	cpu_start = rdtsc();
	for (int i = 0; i < N; i++) {
		int temp = hat->get(i);
	}
	cpu_finish = rdtsc();
	avg_cpu_clocks = (cpu_finish - cpu_start) / N;

    std::cout << "HAT: Avg CPU Clocks:   " << avg_cpu_clocks << std::endl;

	delete hat;
	delete vec;

	cout << endl;
}

/*
 * Benchmarks access latency
 */
void averageAccessBenchmark() {
	HashedArrayTree * hat = new HashedArrayTree();
	DynamicArray * vec = new DynamicArray();

	unsigned long long * hatData = new unsigned long long[N];
	unsigned long long * vecData = new unsigned long long[N];

	for (int i = 0; i < N; i++) {
		vec->push(i);
		hat->push(i);
	}

	cout << "=== INDIVIDUAL SCAN TEST ===" << endl;
	for (int i = 0; i < N; i++) {
		unsigned long long cpu_start = rdtsc();
		vec->get(i);
		unsigned long long cpu_finish = rdtsc();
		vecData[i] = cpu_finish - cpu_start;
	}

	for (int i = 0; i < N; i++) {
		unsigned long long cpu_start = rdtsc();
		hat->get(i);
		unsigned long long cpu_finish = rdtsc();
		hatData[i] = cpu_finish - cpu_start;
	}
	cout << endl;

	fstream hatFile;
	fstream vecFile;
	hatFile.open("../data/hatAvgAccessCycles.txt", ios_base::out);
	vecFile.open("../data/vecAvgAccessCycles.txt", ios_base::out);

	for (int i = 0; i < N; i++) {
		hatFile << hatData[i] << ',';
		vecFile << vecData[i] << ',';
	}

	hatFile.close();
	vecFile.close();

	delete hat;
	delete vec;

	cout << endl;
}

int main() {
	pushBenchmark();
	//averagePushBenchmark();
	//accessBenchmark();
	//averageAccessBenchmark();
}
