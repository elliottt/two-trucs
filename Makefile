
.PHONY: all
all: release

bin:
	mkdir bin

release: | bin
	cargo build --quiet --release && \
		cp target/release/two-trucs bin
