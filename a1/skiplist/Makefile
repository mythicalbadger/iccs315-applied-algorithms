# the compiler: gcc for C program, define as g++ for C++
CC = g++

# compiler flags:
#  -g     - this flag adds debugging information to the executable file
#  -Wall  - this flag is used to turn on most compiler warnings
CFLAGS  = -g -Wall

# The build target 
TARGET = skiplist
ALT_TARGET = skiplist_bidirectional

all: $(TARGET)

$(TARGET): 
	$(CC) $(CFLAGS) src/skiplist/* src/benchmarks.cpp -o $(TARGET)

$(ALT_TARGET):
	$(CC) $(CFLAGS) src/skiplist_bidirectional/* src/benchmarks.cpp -o $(ALT_TARGET)

clean:
	$(RM) $(TARGET) $(ALT_TARGET) data/*.txt
