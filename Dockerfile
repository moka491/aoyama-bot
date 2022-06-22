# ------------------------------------
# Stage 1: Build the application
FROM rust:latest AS builder

WORKDIR /app

RUN cargo install sccache
ENV RUSTC_WRAPPER sccache

COPY . .

RUN cargo build --release

# ------------------------------------
# Stage 2: Build the final container
#
FROM bitnami/minideb:latest
COPY --from=builder /app/target/release/aoyama-bot /usr/local/bin/aoyama-bot
CMD ["aoyama-bot"]