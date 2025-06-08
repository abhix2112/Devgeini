# Use a newer Rust version that supports latest crates
FROM rust:latest AS builder


WORKDIR /app

# Pre-build just to cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -rf src

# Copy actual source and build
COPY . .
RUN cargo build --release

# Create a smaller final image
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/devgeini /usr/local/bin/devgeini

CMD ["devgeini"]
