# --- Stage 1: Builder ---
FROM golang:1.21-buster AS builder
WORKDIR /app
RUN apt-get update && apt-get install -y \
    build-essential \
    cmake \
    git \
    wget \
    && rm -rf /var/lib/apt/lists/*

COPY go.mod go.sum ./
RUN go mod download

COPY . .
RUN CGO_ENABLED=1 GOOS=linux go build -ldflags="-w -s" -o /usr/local/bin/detector ./cmd/main.go

# --- Stage 2: Runtime ---
FROM alpine:latest
RUN apk --no-cache add tini
WORKDIR /app
COPY --from=builder /usr/local/bin/detector /usr/local/bin/detector
COPY symbolic_logic/rules.dl /usr/local/etc/rules.dl
COPY neural_models/quantized_model.onnx /usr/local/etc/model.onnx

RUN addgroup -S appgroup && adduser -S appuser -G appgroup
USER appuser

EXPOSE 9090
ENTRYPOINT ["/sbin/tini", "--", "/usr/local/bin/detector"]
