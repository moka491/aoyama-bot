# ------------------------------------
# Stage 1: Build the application
FROM docker.io/library/rust:latest AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

# ------------------------------------
# Stage 2: Build the final container
#
FROM docker.io/bitnami/minideb:latest
COPY --from=builder /app/target/release/aoyama-bot /usr/local/bin/aoyama-bot
CMD ["aoyama-bot"]