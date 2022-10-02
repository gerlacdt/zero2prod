.PHONY=dev build test db-connect db-migrate
dev:
	cargo watch -x check -x test -x run

build:
	cargo build

test:
	cargo test

db-migrate:
	SKIP_DOCKER=true ./scripts/init_db.sh

db-connect:
	PGPASSWORD=password psql -h localhost -p 5431 -U postgres -d newsletter
