FROM rust:1.62.0-alpine AS rust-image
WORKDIR /app
RUN apk add --update --no-cache musl-dev pkgconfig openssl-dev clang-dev
ENV RUSTFLAGS="-C target-feature=-crt-static"

FROM rust-image as build
COPY src src
COPY Cargo.* .
RUN cargo build --release

FROM alpine as prod
RUN apk add --update --no-cache libgcc
COPY --from=build /app/target/release/bot .
ENTRYPOINT ["./bot"]
