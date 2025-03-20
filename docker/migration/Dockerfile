FROM rust:1.81-alpine

WORKDIR /app

RUN apk add --no-cache musl-dev libressl-dev

# Copy the Cargo.toml and Cargo.lock files
COPY ./backend/migration/Cargo.toml ./backend/migration/Cargo.lock ./

# Fetch dependencies
RUN cargo fetch

# Copy the actual source files
COPY ./backend/migration/src ./src

# Build the actual application
RUN cargo build --release

# Set the command to run the compiled binary
CMD ["sleep", "infinity"]
