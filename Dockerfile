FROM rust:1.85 as builder

RUN mkdir -p /app
WORKDIR /app

RUN cargo install sqlx-cli

COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    libssl3 \
    ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin

COPY --from=builder /app/target/release/server .
COPY --from=builder /usr/local/cargo/bin/sqlx .
COPY --from=builder /app/migrations ./migrations

COPY entrypoint.sh .
RUN chmod +x entrypoint.sh
ENTRYPOINT ["./entrypoint.sh"]
CMD ["./server"]