#all:
#	docker build . -t rcc-test
#	docker run --rm -it rcc-test cargo build && cargo build --release
#
#test:
#	docker build . -t rcc-test
#	docker run -v $(PWD):/usr/src/rcc/ --rm -it rcc-test cargo t
#
#clean:
#	docker build . -t rcc-test
#	docker run --rm -it rcc-test cargo clean
all:
	@cargo b && cargo b --release

test: clippy
	@cargo t

clippy:
	@cargo clean -p rcc
	@cargo clippy

clean:
	@cargo clean
