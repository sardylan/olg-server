FROM rust:latest AS builder
WORKDIR /project
COPY . .
RUN cargo build --release

FROM debian:trixie AS prod
COPY --from=builder /project/target/release/olg-server /app/olg-server
EXPOSE 7000
USER 1000
ENTRYPOINT ["/app/olg-server"]
