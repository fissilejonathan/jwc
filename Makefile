.PHONY: brun

# build and run
br:
	cargo build --release
	./target/release/jwc $(FLAGS)