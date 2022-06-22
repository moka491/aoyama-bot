FROM docker.io/clux/muslrust:stable AS chef
USER root
RUN cargo install cargo-chef
WORKDIR /app

# capture dependencies
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# build with dependency cache
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

# final minimal image
FROM docker.io/library/alpine AS runtime
RUN addgroup -S myuser && adduser -S myuser -G myuser
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/aoyama-bot /usr/local/bin/
USER myuser
CMD ["/usr/local/bin/aoyama-bot"]