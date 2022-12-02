all: day1

debug: day1-debug

day1-debug: main.rs
	rustc $^ -g -o day1

day1: main.rs
	rustc $^ -o day1

clean:
	rm -f *~ day1
