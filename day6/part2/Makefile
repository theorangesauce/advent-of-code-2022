all: day6

debug: day6-debug

day6-debug: main.rs
	rustc $^ -g -o day6

day6: main.rs
	rustc $^ -o day6

clean:
	rm -f *~ day6
