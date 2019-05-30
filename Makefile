# Build the project, tests, or documentation.
.PHONY:
	all test docs publish clean

all:
	cargo build --release && ./target/release/nnet

test:
	cargo test

# works with https://github.com/DanielSchuette/bin/fox
docs:
	cargo doc && fox -t target/doc/nnet/all.html

publish:
	cargo publish

clean:
	cargo clean

