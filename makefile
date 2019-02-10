all:
	cargo build --release

test:
	docker build . -t rcc-test
	docker run --rm rcc-test

clean:
	cargo clean
