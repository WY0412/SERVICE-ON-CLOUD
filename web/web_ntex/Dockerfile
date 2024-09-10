# Use a Rust base image
FROM rust:latest as builder

EXPOSE 80
WORKDIR /usr/src/app
COPY . .

RUN cargo install --path .

CMD ["blockchain"]