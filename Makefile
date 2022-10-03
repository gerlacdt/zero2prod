.PHONY=dev build test db-connect db-migrate
build:
	cargo build

dev:
	cargo watch -x check -x test -x run

test:
	cargo test

db-run:
	SKIP_DOCKER=true ./scripts/init_db.sh

db-migrate:
	./scripts/init_db.sh

db-connect:
	PGPASSWORD=password psql -h localhost -p 5431 -U postgres -d newsletter
