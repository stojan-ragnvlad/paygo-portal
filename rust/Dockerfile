FROM rust:1.78.0-alpine3.18

RUN apk --no-cache add curl

RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

RUN apk --no-cache add clang

RUN apk --no-cache add build-base

COPY src src

COPY Cargo.toml Cargo.lock .

RUN wasm-pack build --target web -d /public

WORKDIR /public

