FROM rust:1.86-alpine

WORKDIR /app

RUN apk add --no-cache musl-dev libressl-dev postgresql-client

COPY ./backend/migration/Cargo.toml ./

RUN cargo fetch

COPY ./backend/migration/src ./src

RUN --mount=type=cache,target=/usr/local/cargo/registry \
	--mount=type=cache,target=/app/backend/target \
	cargo build --release

COPY ./docker/backend-migration/entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh
