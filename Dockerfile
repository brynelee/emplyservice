FROM rust:1.67.1 as builder
WORKDIR /usr/src/emplyservice
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y libpq5 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/emply /usr/local/bin/emply
COPY --from=builder /usr/src/emplyservice/.env /usr/local/bin/.env
COPY --from=builder /usr/src/emplyservice/log.yaml /usr/local/bin/log.yaml
CMD ["emply"]

