FROM rust:1.78.0-alpine3.18

RUN apk --no-cache add clang

RUN apk --no-cache add build-base

RUN cargo install wasm-pack

WORKDIR rust_build

COPY src src

COPY Cargo.toml .

COPY build.sh .

CMD ["sh", "build.sh"]

