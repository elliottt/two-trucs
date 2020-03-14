
.PHONY: all
all: release

bin:
	@mkdir bin

release: | bin
	@echo "Building two-trucs" && \
		cargo build --quiet --release && \
		cp target/release/two-trucs bin
