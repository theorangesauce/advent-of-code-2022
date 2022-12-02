all: day2

debug: day2-debug

day2-debug: main.rs
	rustc $^ -g -o day2

day2: main.rs
	rustc $^ -o day2

clean:
	rm -f *~ day2
