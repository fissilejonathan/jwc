.PHONY: br

# build and run
br:
	cargo build --release
	./target/release/jwc $(INPUT)