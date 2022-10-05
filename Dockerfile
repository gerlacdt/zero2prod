FROM rust:1.64.0
WORKDIR /app
RUN apt update && apt install lld clang -y
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./target/debug/zero2prod"]
