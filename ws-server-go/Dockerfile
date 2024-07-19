# syntax=docker/dockerfile:1

FROM --platform=$BUILDPLATFORM golang:1.22 AS build

ARG TARGETPLATFORM

WORKDIR /app

COPY . .
RUN go mod download

ENV CGO_ENABLED=0

RUN case "$TARGETPLATFORM" in \
        "linux/amd64")  export GOARCH=amd64 ;; \
        "linux/arm64")  export GOARCH=arm64 ;; \
        *) echo "Unsupported platform: $TARGETPLATFORM"; exit 1 ;; \
    esac && \
    go build -o /app/ws-server-go

FROM gcr.io/distroless/base-debian12

EXPOSE 8080

USER nonroot:nonroot

COPY --from=build --chown=nonroot:nonroot /app/ws-server-go /app/ws-server-go

ENTRYPOINT ["/app/ws-server-go"]
