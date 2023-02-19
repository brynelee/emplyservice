FROM rust:1.67.1

WORKDIR /usr/src/emplyservice
COPY . .

RUN cargo install --path .

EXPOSE 8086

CMD ["emplyservice"]