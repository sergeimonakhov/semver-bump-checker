
ARG RUST_IMAGE=rust:1.77-slim-bookworm

FROM $RUST_IMAGE as builder

WORKDIR /usr/src/app

COPY . .

RUN apt-get update \
    && apt-get install --no-install-recommends -y pkg-config libssl-dev

RUN cargo build --release

# Stage 2: Create a minimal image with just the binary
FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install --no-install-recommends -y libssl-dev \
    && apt-get clean autoclean \
    && apt-get autoremove --yes \
    && rm -rf /var/lib/{apt,dpkg,cache,log}/

# Copy the binary from the previous stage
COPY --from=builder /usr/src/app/target/release/sbc /usr/local/bin/
