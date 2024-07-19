# syntax=docker/dockerfile:1

FROM --platform=$BUILDPLATFORM rust:1.79.0-slim-bookworm AS build

ARG TARGETPLATFORM

RUN apt-get update \
    && apt-get install -y \
        build-essential \
        pkg-config \
        cmake \
        libc-bin \
        libc6-dev \
        libc6-dev-arm64-cross \
        gcc \
        musl \
        musl-tools \
        gcc-aarch64-linux-gnu \
    && rm -rf /var/lib/apt/lists/* \
    && update-ca-certificates

WORKDIR /app

COPY . .

ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc
ENV CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc

RUN case "${TARGETPLATFORM}" in \
      "linux/amd64") rust_target="x86_64-unknown-linux-musl" ;; \
      "linux/arm64") rust_target="aarch64-unknown-linux-gnu" ;; \
      *) echo "Unsupported platform: ${TARGETPLATFORM}" ; exit 1 ;; \
    esac \
    && rustup target add ${rust_target} \
    && cargo install --target ${rust_target} --path .

FROM gcr.io/distroless/base-debian12

EXPOSE 8081

USER nonroot:nonroot

COPY --from=build --chown=nonroot:nonroot /usr/local/cargo/bin/ws-server-rs /app/ws-server-rs

ENTRYPOINT ["/app/ws-server-rs"]
