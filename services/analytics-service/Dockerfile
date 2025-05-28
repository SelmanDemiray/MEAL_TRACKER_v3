FROM rust:1.82 as builder

WORKDIR /app
COPY Cargo.toml ./
RUN cargo fetch

COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/analytics-service /app/analytics-service

EXPOSE 8082

CMD ["./analytics-service"]
