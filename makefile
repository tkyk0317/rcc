all:
	cargo build --release

debug:
	cargo build

test:
	cargo t && ./test.sh

clean:
	cargo clean
