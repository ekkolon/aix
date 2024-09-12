FROM rust:latest AS builder

WORKDIR /usr/src/app
COPY . /usr/src/app

# Build and cache the binary and dependent crates in release mode
RUN --mount=type=cache,target=/usr/local/cargo,from=rust:latest,source=/usr/local/cargo \
    --mount=type=cache,target=target \
    cargo build --release && mv ./target/release/{{crate_name}} ./{{crate_name}}

# Debian-based 'distroless' runtime image
FROM gcr.io/distroless/cc-debian12

ENV DOCKER_RUNNING=true

WORKDIR /app

# Copy compiled binary from builder's cargo install directory
COPY --from=builder /usr/src/app/{{crate_name}} ./{{crate_name}}

CMD ["./{{crate_name}}"]