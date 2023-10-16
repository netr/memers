.PHONY: build
build:
	cargo build --release

.PHONY: test
test:
	cargo test -- --nocapture

.PHONY: run
run:
	cargo run

.PHONY: runr
runr: build
	./target/release/memers

.PHONY: clean
clean:
	cargo clean

.PHONY: doc
doc:
	cargo doc

.PHONY: update
update:
	cargo update