FROM rust:1.55.0 AS build-env
WORKDIR /app
COPY . /app
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=build-env /app/target/release/fizzbuzz-extreme-edition /bin
CMD [ "fizzbuzz-extreme-edition" ]
