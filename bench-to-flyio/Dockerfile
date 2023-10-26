FROM rust:1.70.0 as builder

WORKDIR /app

COPY .. .
RUN cargo build --bin bench --release

FROM debian:bullseye-slim AS runner
COPY --from=builder /app/target/release/bench /usr/local/bin
