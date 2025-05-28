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

COPY --from=builder /app/target/release/nutrition-service /app/nutrition-service

EXPOSE 8081

CMD ["./nutrition-service"]
