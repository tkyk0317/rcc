all:
	docker build . -t rcc-test
	docker run --rm -it rcc-test cargo build && cargo build --release

test:
	docker build . -t rcc-test
	docker run -v $(PWD):/usr/src/rcc/ -it rcc-test cargo t

clean:
	docker build . -t rcc-test
	docker run --rm -it rcc-test cargo clean
