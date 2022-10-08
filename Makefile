.PHONY=dev build test db-connect db-migrate docker-build docker-run
build:
	cargo build

dev:
	cargo watch -x check -x test

run:
	cargo run

test:
	cargo test

db-migrate:
	SKIP_DOCKER=true ./scripts/init_db.sh

db-run:
	./scripts/init_db.sh

db-connect:
	PGPASSWORD=password psql -h localhost -p 5431 -U postgres -d newsletter


docker-build:
	docker build -t zero2prod .

docker-run:
	docker run --network=host -p 8000:8000 zero2prod:latest
