.PHONY=dev build test
dev:
	cargo watch -x check -x test -x run

db-migrate:
	SKIP_DOCKER=true ./scripts/init_db.sh

build:
	cargo build

test:
	make test
