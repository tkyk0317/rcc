all:
	docker build . -t rcc-test
	docker run --rm -it rcc-test cargo build && cargo build --release

test:
	docker build . -t rcc-test
	docker run -v $(PWD):/usr/src/rcc/ -t rcc-test cargo clippy && cargo t

clippy:
	docker build . -t rcc-test
	docker run --rm -t rcc-test cargo clippy

clean:
	docker build . -t rcc-test
	docker run --rm -t rcc-test cargo clean
