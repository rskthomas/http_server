
FROM rust:1.67-alpine AS builder
WORKDIR /usr/src/http_server
COPY . .

EXPOSE 80
RUN cargo install --path .
CMD ["cargo", "run", "--release"]
