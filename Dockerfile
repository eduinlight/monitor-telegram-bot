FROM rust:1.62.0 AS rust-image
WORKDIR /app
RUN apt-get update && \
      apt-get install -yq libclang-dev

FROM rust-image as build
COPY src src
COPY Cargo.* .
RUN cargo build --release

FROM build AS prod
COPY --from=build /app/target/release .
CMD ["./bot"]
