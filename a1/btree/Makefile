# the compiler: gcc for C program, define as g++ for C++
CC = g++

# compiler flags:
#  -g     - this flag adds debugging information to the executable file
#  -Wall  - this flag is used to turn on most compiler warnings
CFLAGS  = -g -Wall

# The build target 
TARGET = btree

all: $(TARGET)

$(TARGET): 
	$(CC) $(CFLAGS) src/btree/* src/btreenode/* src/benchmarks.cpp -o $(TARGET)

clean:
	$(RM) $(TARGET)
