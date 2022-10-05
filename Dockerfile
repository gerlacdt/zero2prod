FROM rust:1.64.0 AS builder

WORKDIR /app
RUN apt update && apt install lld clang -y
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build
ENV APP_ENVIRONMENT production

FROM debian:bullseye-slim AS runtime
WORKDIR /app
RUN apt-get update -y \
&& apt-get install -y --no-install-recommends openssl ca-certificates \
&& apt-get autoremove -y \
&& apt-get clean -y \
&& rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/debug/zero2prod zero2prod
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./zero2prod"]
