all: day5

debug: day5-debug

day5-debug: main.rs
	rustc $^ -g -o day5

day5: main.rs
	rustc $^ -o day5

clean:
	rm -f *~ day5
