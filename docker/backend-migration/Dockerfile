FROM rust:1.81-alpine

WORKDIR /app

RUN apk add --no-cache musl-dev libressl-dev

# Copy the Cargo.toml files
COPY ./backend/migration/Cargo.toml ./

# Fetch dependencies
RUN cargo fetch

# Copy the actual source files
COPY ./backend/migration/src ./src

# Build the actual application
RUN --mount=type=cache,target=/usr/local/cargo/registry \
	--mount=type=cache,target=/app/backend/target \
	cargo build --release


# Set the command to run the compiled binary
CMD ["sleep", "infinity"]
