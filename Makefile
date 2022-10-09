.PHONY=dev db-migrate db-run db-connect docker-build docker-run
dev:
	cargo watch -x check -x clippy -x "test -- --test-threads 1"

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
