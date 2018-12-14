all:
	cargo build --release

debug:
	cargo build

test:
	cargo t && ./test.sh

sample:
	cargo b && ./target/debug/rcc ./example/test.c > ./out.s && gcc out.s -o a.out && ./a.out ; echo $$?
	rm out.s a.out

clean:
	cargo clean
