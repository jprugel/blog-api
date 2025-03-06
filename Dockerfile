FROM rust:latest AS builder
WORKDIR /usr/src/blog-api
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get upgrade && apt-get install -y libpq5 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/blog-api /usr/local/bin/blog-api

CMD ["blog-api"]
