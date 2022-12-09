all: day9

debug: day9-debug

day9-debug: main.rs
	rustc $^ -g -o day9

day9: main.rs
	rustc $^ -o day9

clean:
	rm -f *~ day9
