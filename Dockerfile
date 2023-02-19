FROM rust:1.67.1 as builder
WORKDIR /usr/src/emplyservice
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y libpq5 && rm -rf /var/lib/apt/lists/*
WORKDIR /usr/src/emplyservice
COPY --from=builder /usr/local/cargo/bin/emply /usr/src/emplyservice/emply
COPY --from=builder /usr/src/emplyservice/.env /usr/src/emplyservice/.env
COPY --from=builder /usr/src/emplyservice/log.yaml /usr/src/emplyservice/log.yaml
CMD ["/usr/src/emplyservice/emply"]

