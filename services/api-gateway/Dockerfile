FROM rust:1.82 as builder

WORKDIR /app
COPY Cargo.toml ./
RUN cargo fetch

COPY src ./src
COPY migrations ./migrations

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/api-gateway /app/api-gateway

EXPOSE 8080

CMD ["./api-gateway"]
