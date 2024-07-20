FROM rust:1.78.0-alpine3.18

RUN apk --no-cache add clang

RUN apk --no-cache add build-base

RUN cargo install wasm-pack

WORKDIR rust_build

COPY src src

# ARG application_url

ARG PP_BUILD_TYPE

COPY build.sh .

# RUN sh build.sh $application_url

COPY Cargo.toml .

RUN sh build.sh

COPY run.sh .

CMD sh run.sh

