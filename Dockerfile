FROM rust:latest AS builder
WORKDIR /project
COPY . .
RUN cargo build --release

FROM debian:trixie AS prod
COPY --from=builder /project/target/release/olg-server /app/olg-server