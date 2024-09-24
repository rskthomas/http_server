
FROM rust:1.67-alpine AS builder
WORKDIR /usr/src/http_server
COPY . .

EXPOSE 8080
RUN cargo install --path .
CMD ["cargo", "run", "--release"]
