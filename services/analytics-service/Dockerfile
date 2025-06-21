FROM rust:1.85 as builder

WORKDIR /app

# Copy the Cargo manifests
COPY Cargo.toml ./

# Build dummy project to cache dependencies
RUN mkdir -p src && \
    echo "fn main() {println!(\"Dummy main\")}" > src/main.rs && \
    cargo build && \
    rm -rf src target

# Copy the actual source code
COPY . .

# Build the actual project
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from the build stage
COPY --from=builder /app/target/release/analytics-service .

EXPOSE 8082

CMD ["./analytics-service"]
