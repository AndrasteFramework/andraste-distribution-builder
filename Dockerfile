FROM rust:1.89 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

# FROM alpine:latest # TODO: See below
FROM debian:stable-slim
WORKDIR /app
COPY --from=builder /app/target/release/andraste-distribution-builder /app/
# RUN apk --no-cache add libc6-compat libgcc # TODO: maybe compile against musl to begin with? but that compile errors.
RUN DEBIAN_FRONTEND=noninteractive apt update && apt -y install ca-certificates && apt clean
ENV RUST_BACKTRACE=1
ENTRYPOINT ["./andraste-distribution-builder"]
CMD []
