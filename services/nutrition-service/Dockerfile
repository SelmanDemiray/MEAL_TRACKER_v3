FROM rust:1.85 as builder

WORKDIR /app

# Copy only the Cargo.toml file first
COPY Cargo.toml ./

# Create a dummy main.rs file to build dependencies
RUN mkdir -p src && \
    echo "fn main() {println!(\"Dummy main\")}" > src/main.rs && \
    cargo build && \
    rm -rf src target

# Now copy the actual source code
COPY . .

# Build the application
RUN cargo build --release

FROM debian:bookworm-slim

# Install necessary dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    tzdata \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/nutrition-service ./nutrition-service

# Expose the port the service runs on
EXPOSE 8081

# Run the service
CMD ["./nutrition-service"]
