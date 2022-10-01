.PHONY=dev build test
dev:
	cargo watch -x check -x test -x run

build:
	cargo build

test:
	make test
