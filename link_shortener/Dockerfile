FROM rust:1.83.0 AS builder
WORKDIR /usr/src/shortener
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libc6 libssl3 ca-certificates curl && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/shortener /usr/local/bin/shortener
CMD ["shortener"]